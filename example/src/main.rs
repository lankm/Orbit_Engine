#![allow(unused)]
#![allow(non_snake_case)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };
use std::mem::size_of;
use std::f32::consts::PI;

use example::*;


fn main() {
    
}
fn something() {
    const COUNT: i32 = 1000;
    const STEP: f32 = (2.0*PI/(COUNT as f32));

    for i in 0..COUNT {
        let M = STEP*(i as f32);
        let pos = calc_pos(M, 2.0, 1.0);
        println!("({}, {})", pos.0, pos.1);
    }
}