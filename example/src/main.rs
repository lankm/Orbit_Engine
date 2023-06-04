#![allow(unused)]

use rand::Rng;
use std::*;
use uom::si::f64::*;
use uom::si::time::{zettasecond, second};


fn objects() {
    // objects inside modules are implicitly private.
    mod module {
        // data
        pub struct Object<Generic> {
            val: Generic,
        }
        // functions
        impl<Generic> Object<Generic> {
            pub fn new(num: Generic) -> Object<Generic> {
                return Object {
                    val: num,
                }
            }
            pub fn get(&self) -> u32 {
                return 15;
            }
        }
    }
    use module::Object;
    // struct
    let mut o = Object::new(32);
    println!("{}",o.get());
}
fn primitives() -> i32 {
    let int: i64; // i8 i16 i32 i64 i128 isize
    let uint: i64; // u8 u16 u32 u64 u128 usize
    let float: f32; // f32 f64
    let bool: bool; // bool
    let char: char; // char
    // tuple
    let tuple: (u32, f32, i8) = (0,2.1,5);
    tuple.0;
    // array
    let array: [i32; 5] = [1,2,3,4,5]; 
    let array = [3; 5]; // [3,3,3,3,3]
    return array[0];
}

fn main() {
    let time = Time::new::<zettasecond>(1.0);
    let time_c = time.get::<second>();
    println!("{}", time_c);
}
