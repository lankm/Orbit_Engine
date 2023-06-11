#![allow(unused)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };
use std::mem::size_of;
use std::fmt;

struct Test {
    x: i128,
}
impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let power = 20;
        let whole = self.x >> power;
        let decimal = self.x - (whole << power);
        write!(f, "{}.{:0>6}", whole, decimal)
    }
}


fn main() {
    let mut s = Test{ x:127984<<20 };
    s.x -= 1;
    println!("{s}");
}
