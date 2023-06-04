#![allow(unused)]

use std::*;

mod module {
    // things within a module are implicitly private to those outside of it.
    // object
    pub struct Object<Generic> {
        val: Generic,
    }
    // object implementation
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
    // enum
    pub enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Day {
        pub fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
}
fn main() {
    use module::Object;
    use module::Day;

    // object
    let mut o = Object::new(32);
    println!("{}",o.get());
    // enum
    let d = Day::Tuesday;
    println!("{}", d.is_weekend());
}
