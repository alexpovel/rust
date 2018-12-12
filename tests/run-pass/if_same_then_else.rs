// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(clippy::if_same_then_else)]

fn main() {}

pub fn foo(a: i32, b: i32) -> Option<&'static str> {
    if a == b {
        None
    } else if a > b {
        Some("a pfeil b")
    } else {
        None
    }
}
