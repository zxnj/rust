// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub enum X {
    Y
}

pub struct Z {
    x: X
}

pub fn main() {
    let z = Z { x: X::Y };
    let _ = &mut z.x;
}

pub fn with_arg(z: Z, w: &Z) {
    let _ = &mut z.x;
    let _ = &mut w.x;
}
