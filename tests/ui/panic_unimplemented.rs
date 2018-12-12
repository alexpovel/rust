// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(clippy::panic_params, clippy::unimplemented)]

fn missing() {
    if true {
        panic!("{}");
    } else if false {
        panic!("{:?}");
    } else {
        assert!(true, "here be missing values: {}");
    }

    panic!("{{{this}}}");
}

fn ok_single() {
    panic!("foo bar");
}

fn ok_inner() {
    // Test for #768
    assert!("foo bar".contains(&format!("foo {}", "bar")));
}

fn ok_multiple() {
    panic!("{}", "This is {ok}");
}

fn ok_bracket() {
    match 42 {
        1337 => panic!("{so is this"),
        666 => panic!("so is this}"),
        _ => panic!("}so is that{"),
    }
}

const ONE: u32 = 1;

fn ok_nomsg() {
    assert!({ 1 == ONE });
    assert!(if 1 == ONE { ONE == 1 } else { false });
}

fn ok_escaped() {
    panic!("{{ why should this not be ok? }}");
    panic!(" or {{ that ?");
    panic!(" or }} this ?");
    panic!(" {or {{ that ?");
    panic!(" }or }} this ?");
    panic!("{{ test }");
    panic!("{case }}");
}

fn unimplemented() {
    let a = 2;
    unimplemented!();
    let b = a + 2;
}

fn main() {
    missing();
    ok_single();
    ok_multiple();
    ok_bracket();
    ok_inner();
    ok_nomsg();
    ok_escaped();
    unimplemented();
}
