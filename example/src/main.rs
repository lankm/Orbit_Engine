#![allow(unused)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };

fn main() {
    let vec = vec![1,2,5,4];
    let vec_deque = VecDeque::from([1,2,3]);
    let bin_tree_set = BTreeSet::from([0,4,2,6,-5]);
    let bin_heap = BinaryHeap::from([0,5,2,8,2,6,4,9]);

    let c = bin_tree_set.into_iter();
    // for n in bin_tree_set.iter() {
    //     println!("{n}");
    // }
}
