#![allow(unused)]

use std::*;

fn main() {
    let int: i64 = -1; // i8 i16 i32 i64 i128 isize
    let uint: u64 = int as u64; // u8 u16 u32 u64 u128 usize. casting
    let float: f32 = 0.0; // f32 f64
    let bool: bool = false; // bool
    let char: char = 'c'; // char
    // tuple
    let tuple: (u32, f32, i8) = (0,2.1,5);
    tuple.0;
    // array
    let array: [i32; 5] = [1,2,3,4,5]; 
    let array = [3; 5]; // [3,3,3,3,3]
    // string
    let string: String = String::new();
    let str: &str = "";
}
