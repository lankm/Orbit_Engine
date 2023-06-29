/* README
 * This implementation is for numbers whose overflows do not matter.
 * A good example is an angle where 2pi = 0. 10 rotations is the same position as 0.
 * For the case of an angle, the most significant bit is equal to pi.
 * The rest har inverse powers of two. An overflow is equal to 2pi.
 */

#![allow(non_camel_case_types)]

use core::fmt;
use std::ops;
use std::f64::consts::PI;

pub struct r32 {
    angle: u32,
}
impl r32 {
    pub fn new(mut num: f64) -> r32 {
        return r32{ angle: 0 };
    }
}