#![allow(unused)]

use core::fmt;
// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };
use std::mem::size_of;

struct Test {
    x: i32,
    y: i32,
}
impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
    let s = Test{ x:2, y:5 };
    println!("{s}");
}
