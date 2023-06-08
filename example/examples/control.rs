#![allow(unused)]

use std::*;
use std::cmp;

fn main() {
    let number = 3;

    // if else
    if number < 5 {
    } else if number > 5 {
    } else {
    }

    // ternary
    let num = if number == 5 {1} else {2};

    // match
    match number {
        1..=10 => println!("1"),
        11 | 12 => println!("2"),
        _ => println!("3"),
    }
}
