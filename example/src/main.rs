#![allow(unused)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };

fn main() {
    search_insert(vec![0,2,3,4,6,7,8,9,20], 15);
}
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let i = nums.binary_search(&target);
    match i {
        Ok(x) => x as i32,
        Err(x) => x as i32,
    }
}
