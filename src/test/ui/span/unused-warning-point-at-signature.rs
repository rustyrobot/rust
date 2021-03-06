// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass

#![warn(unused)]

enum Enum {
    A,
    B,
    C,
    D,
}

struct Struct {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

fn func() -> usize {
    3
}

fn
func_complete_span()
-> usize
{
    3
}

fn main() {}
