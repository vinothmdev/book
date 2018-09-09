// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Examples are inspired from doc.rust-lang.org
// இந்த நிரல் பலவகையன சொற்றொடர் வடிவமைக்கும் உதாரனங்களை கொன்டுள்ளது
fn main() {
    // பொதுவாக {} அனைத்து தருமதிப்புகளுக்கும் இடமளிக்கும், அவை அனைத்தும் சொற்றொடராக
    // வடிவமைக்க படும் (stringified)
    println!("{} days", 31);

    // இடம் சார்ந்த தருமதிப்புகளும் ஏற்றுகொள்ளப்படும்
    println!("{0} பின் {1}", "ஆடி", "ஆவனி");

    // தருமதிப்புகளை அடயால பெயருடன் வடிவமைக்க இயலும்
    println!("{subject} {verb} {object}",
             object = "இடம்",
             subject = "பொருள்",
             verb = "ஏவல்");

    println!("{} of {:b}", 1, 0xa);
    // This is different than java where formatted string are not checked
    // During compile time, if you try to compile following code Rust will
    // Through error similar to C++
    // println!("{number:=width$}*", number=1, width=20);
    // FIXME ^ uncomment this line to test
    // This will work with out compilation error
    println!("{number:>width$}", number=1, width=20);

    // print with padding
    println!("{number:>0width$}", number=1, width=6);

    // floating point display with controlled decimal place
    println!("{:.*}", 2, 1.23456);
}
