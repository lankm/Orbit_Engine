#![allow(non_camel_case_types)]

use core::fmt;
use std::ops;
use std::f64::consts::PI;

pub struct r16 {
    angle: u16,
}
impl r16 {
    pub fn new(mut num: f64) -> r16 {
        num %= 2.0*PI;
        num *= u16::MAX as f64;
        num /= 2.0*PI;
        r16 { angle: num as u16, }
    }
}
impl ops::Add<r16> for r16 {
    type Output = r16;

    fn add(self, theta: r16) -> r16 {
        r16 { angle: self.angle + theta.angle }
    }
}
impl fmt::Display for r16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: f64 = self.angle as f64;
        out /= u16::MAX as f64;
        out *= 2.0*PI;

        write!(f, "{}", out)
    }
}