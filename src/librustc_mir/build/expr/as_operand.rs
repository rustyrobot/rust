// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! See docs in build/expr/mod.rs

use build::{BlockAnd, BlockAndExtension, Builder};
use build::expr::category::Category;
use hair::*;
use rustc::middle::region;
use rustc::mir::*;

impl<'a, 'gcx, 'tcx> Builder<'a, 'gcx, 'tcx> {
    /// Returns an operand suitable for use until the end of the current
    /// scope expression.
    ///
    /// The operand returned from this function will *not be valid* after
    /// an ExprKind::Scope is passed, so please do *not* return it from
    /// functions to avoid bad miscompiles.
    pub fn as_local_operand<M>(&mut self, block: BasicBlock, expr: M)
                             -> BlockAnd<Operand<'tcx>>
        where M: Mirror<'tcx, Output = Expr<'tcx>>
    {
        let local_scope = self.local_scope();
        self.as_operand(block, local_scope, expr)
    }

    /// Compile `expr` into a value that can be used as an operand.
    /// If `expr` is an lvalue like `x`, this will introduce a
    /// temporary `tmp = x`, so that we capture the value of `x` at
    /// this time.
    ///
    /// The operand is known to be live until the end of `scope`.
    pub fn as_operand<M>(&mut self,
                         block: BasicBlock,
                         scope: Option<region::Scope>,
                         expr: M) -> BlockAnd<Operand<'tcx>>
        where M: Mirror<'tcx, Output = Expr<'tcx>>
    {
        let expr = self.hir.mirror(expr);
        self.expr_as_operand(block, scope, expr)
    }

    fn expr_as_operand(&mut self,
                       mut block: BasicBlock,
                       scope: Option<region::Scope>,
                       expr: Expr<'tcx>)
                       -> BlockAnd<Operand<'tcx>> {
        debug!("expr_as_operand(block={:?}, expr={:?})", block, expr);
        let this = self;

        if let ExprKind::Scope { region_scope, lint_level, value } = expr.kind {
            let source_info = this.source_info(expr.span);
            let region_scope = (region_scope, source_info);
            return this.in_scope(region_scope, lint_level, block, |this| {
                this.as_operand(block, scope, value)
            });
        }

        let category = Category::of(&expr.kind).unwrap();
        debug!("expr_as_operand: category={:?} for={:?}", category, expr.kind);
        match category {
            Category::Constant => {
                let constant = this.as_constant(expr);
                block.and(Operand::Constant(box constant))
            }
            Category::Lvalue |
            Category::Rvalue(..) => {
                let operand =
                    unpack!(block = this.as_temp(block, scope, expr));
                block.and(Operand::Consume(Lvalue::Local(operand)))
            }
        }
    }
}
