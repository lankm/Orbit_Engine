#![allow(non_snake_case)]
#![allow(unused)]

use std::f32::consts::PI;

pub struct Orbit {
    e: f32, // eccentricity
    a: f32, // semimajor axis
        b: f32, // semiminor axis

    i: f32, // inclination
    o: f32, // longitude of the ascending node
    w: f32, // argument of periapsis

    t0: f32, // time of periapsis passage
}
impl Orbit {
    pub fn new( e: f32, a: f32, i: f32, o: f32, w: f32, t0: f32 ) -> Orbit {
        return Orbit { 
            e, 
            a, 
            b: Orbit::b(e, a), 
            i, 
            o, 
            w,
            t0,
        }
    }

    pub fn E(&self, M: f32) -> f32 {
        let mut E: f32 = 0.0;

        for i in 0..1000 {
            E = M + self.e*E.sin();
        }

        return E;
    }
    pub fn pos(&self, M: f32) -> (f32, f32) {
        let pos_2d = self.pos_2d(M); // simple 2d
        let pos_rot = self.pos_rot(pos_2d); // apply argument of periapsis
        // let pos_inc = self.pos_inc(pos_rot);
        return pos_rot;
    }
    fn pos_2d(&self, M: f32) -> ( f32, f32 ) {
        let E = self.E(M);
    
        let x = self.a*(E.cos()-self.e);
        let y = self.b*E.sin();

        return ( x, y );
    }
    fn pos_rot(&self, pos_2d: (f32, f32)) -> (f32, f32) {
        let mut x = pos_2d.0;
        let mut y = pos_2d.1;
        
        let r = (x*x+y*y).sqrt();
        let mut theta = (y/x).atan() + self.w;
        theta += if x<0.0 {PI} else {0.0}; // atan correction

        x = r*theta.cos();
        y = r*theta.sin();
        return ( x, y );
    }
    // fn pos_inc(&self, pos_rot: (f32, f32)) -> (f32, f32, f32) {
        
    // }

    fn a(e: f32, b: f32) -> f32 {
        return b*( 1.0/(1.0-e*e) ).sqrt();
    }
    fn b(e: f32, a: f32) -> f32 {
        return a*(1.0-e*e).sqrt();
    }
    fn e(a: f32, b: f32) -> f32 {
        return (1.0-( (b*b)/(a*a) )).sqrt();
    }
    
    pub fn apoapsis(&self) -> f32 {
        return self.a - self.periapsis();
    }
    pub fn periapsis(&self) -> f32 {
        return ( self.a - self.a*self.e )/2.0
    }
}