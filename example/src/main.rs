#![allow(unused)]

use std::collections::{HashMap, BinaryHeap};

fn main() {
    let vec = vec![1,2,5,4];
    let heap = BinaryHeap::from(vec);

    for n in heap.iter() {
        println!("{n}");
    }
}
