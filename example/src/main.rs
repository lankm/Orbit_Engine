#![allow(unused)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };
use std::mem::size_of;

struct Test {
    x: i32,
    y: i32,
}


fn main() {
    println!("{}", size_of::<Test>());
}
