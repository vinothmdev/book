// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("X = {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("length = {}", spaces);

    for number in (1..10).rev() {
        println!("Number {}", number);
    }

    for number in [1,2,3,4,5].iter().rev() {
        println!("Number {}", number);
    }
}
