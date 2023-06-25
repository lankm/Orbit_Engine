#![allow(non_snake_case)]
#![allow(unused)]

pub mod angle;

use std::f32::consts::PI;

pub fn rot_x(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let x = pos.0;
    let mut y = pos.1;
    let mut z = pos.2;

    if z==0.0 && y==0.0 {
        return (x, y, z);
    }
        
    let r = (y*y+z*z).sqrt(); // radius from axis
    let mut theta = (z/y).atan(); // angle from plane
    theta += if y<0.0 {PI} else {0.0}; // atan correction
    theta += angle; // applying rotating

    y = r*theta.cos();
    z = r*theta.sin();
    return ( x, y, z );
}
pub fn rot_y(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let mut x = pos.0;
    let y = pos.1;
    let mut z = pos.2;

    if x==0.0 && z==0.0 {
        return (x, y, z);
    }
        
    let r = (x*x+z*z).sqrt(); // radius from axis
    let mut theta = (x/z).atan(); // angle from axis
    theta += if z<0.0 {PI} else {0.0}; // atan correction
    theta += angle; // applying rotating

    z = r*theta.cos();
    x = r*theta.sin();
    return ( x, y, z );
}
pub fn rot_z(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let mut x = pos.0;
    let mut y = pos.1;
    let z = pos.2;

    if x==0.0 && y==0.0 {
        return (x, y, z);
    }
        
    let r = (x*x+y*y).sqrt(); // radius from axis
    let mut theta = (y/x).atan(); // angle from axis
    theta += if x<0.0 {PI} else {0.0}; // atan correction
    theta += angle; // applying rotating

    x = r*theta.cos();
    y = r*theta.sin();
    return ( x, y, z );
}

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

    fn E(&self, M: f32) -> f32 {
        let mut E: f32 = 0.0;

        for i in 0..1000 {
            E = M + self.e*E.sin();
        }

        return E;
    }
    pub fn pos(&self, M: f32) -> (f32, f32, f32) {
        let E = self.E(M);
        let mut pos = self.pos_elliptic(E);
        pos = rot_z(pos, self.w); // apply argument of periapsis
        pos = rot_x(pos, self.i); // apply inclination
        pos = rot_z(pos, self.o); // apply longitude of the ascending node
        return pos;
    }
    fn pos_elliptic(&self, E: f32) -> ( f32, f32, f32 ) {
        let x = self.a*(E.cos()-self.e);
        let y = self.b*E.sin();

        return ( x, y, 0.0 );
    }

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
