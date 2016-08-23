// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

fn non_elidable<'a, 'b>(a: &'a u8, b: &'b u8) -> &'a u8 { a }

// the boundaries of elision
static NON_ELIDABLE_FN : &fn(&u8, &u8) -> &u8 = non_elidable;
//~^ERROR: missing lifetime specifier

fn main() {
    // nothing to do here
}
