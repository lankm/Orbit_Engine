#![allow(unused)]
#![allow(non_snake_case)]

use kepler::*;
use std::{ops::{Add, Mul}, time::Instant};
use kepler::angle::r16;

fn main() {
    test()
}
fn pi_test() {
    use std::f64::consts::PI;

    let pi = 1 as f64;
    let mut one = pi/PI;

    let mut min_idx = -1;
    let mut min = 1.0;

    for i in 0..47 {
        let delta = if one>0.5 {1.0-one} else {one};
        if delta<min {
            min = delta;
            min_idx = i;
        }

        println!("{}:    {}", i, delta);
        one *= 2.0;
        one %= 1.0;
    }
    println!("MIN: {} with {}", min_idx, min);
}
fn test() {
    use std::f32::consts::PI;
    let orbit = Orbit::new(0.8, 5.0, PI/4.0, PI*3.0/2.0, 0.0, 0.0);

    const COUNT: i32 = 10;
    const STEP: f32 = (2.0*PI/(COUNT as f32));

    for i in 0..COUNT {
        let M = STEP*(i as f32);
        let pos = orbit.pos(M);
        println!("({}, {}, {})", pos.0, pos.1, pos.2);
    }
}