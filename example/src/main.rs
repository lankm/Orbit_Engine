#![allow(unused)]
#![allow(non_snake_case)]

// https://doc.rust-lang.org/std/collections/index.html
use std::collections::{ BTreeMap, BTreeSet, BinaryHeap, VecDeque, LinkedList, HashMap, HashSet };
use std::mem::size_of;
use std::f32::consts::PI;

fn main() {
    const COUNT: i32 = 1000;
    const STEP: f32 = (2.0*PI/(COUNT as f32));

    for i in 0..COUNT {
        let M = STEP*(i as f32);
        let pos = calc_pos(M, 2.0, 1.0);
        println!("({}, {})", pos.0, pos.1);
    }
}

fn calc_E(M: f32, e: f32) -> f32 {
    let mut E: f32 = 0.0;

    for i in 0..1000 {
        E = M + e*E.sin();
    }

    return E;
}
fn calc_e(a: f32, b: f32) -> f32 {
    let e = (1.0-((b*b)/(a*a))).sqrt();
    return e;
}
fn calc_pos(M: f32, a: f32, b: f32) -> (f32, f32) {
    let e = calc_e(a, b);
    let E = calc_E(M, e);

    let x = a*(E.cos()-e);
    let y = b*E.sin();
    return ( x, y );
}