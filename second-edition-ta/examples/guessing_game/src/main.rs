// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Everything starts with main
fn main() {
    // Creating random number
    // Refer how carot rand is added to cargo.toml
    let secret_number = rand::thread_rng().gen_range(0, 101);
    loop {
        // Printing helper messages to get game started
        println!("Guess the number!");
        println!("Please input your guess.");

        // Create mutable variable to hold new inputs from user
        // by default rust create immutable constants the mut keyword does the conversion
        let mut guess = String::new();

        // Get associated stdin method from standard package, using which call read_line method
        // By using &mut make mutable reference guess
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing variable with different type and handling error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // compare guess and exit loop
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Equal!!!");
                break;
            }
        }
    }
}
