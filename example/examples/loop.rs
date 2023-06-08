#![allow(unused)]

use std::*;

fn main() {
    let mut number = 5;

    // infinite loop with label
    'a: loop {
        if number == 0 {
            break 'a;
        }
        print!("{number} ");
        number-=1;
        continue; // not needed
    }
    println!("infinite loop");

    // while loop
    number = 5;
    while number > 0 {
        print!("{number} ");
        number-=1;
    }
    println!("while");

    // do while
    number = 5;
    while {
        print!("{number} ");
        number -= 1;
        number > 0
    } {}
    println!("do while");

    // while let loop
    let mut x = vec![1, 2, 3, 4, 5];
    while let Some(y) = x.pop() {
        print!("{y} ");
    }
    println!("while let");

    // for loop
    number = 5;
    for y in (1..number+1).rev() {
        print!("{y} ");
    }
    println!("for");

    //for each loop
    x = vec![5, 4, 3, 2, 1];
    for y in x {
        print!("{y} ");
    }
    println!("foreach");

}
