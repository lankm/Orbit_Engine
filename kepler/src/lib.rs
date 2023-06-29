/* README
 * This is a set of orbital parameters. Currently everything is floats, but
 * This should change to fixed point numbers due to the loss of precision.
 * Space has no center so it should not be more accurate depending on the time
 * or location.
 */
#![allow(non_snake_case)]
#![allow(unused)]

pub mod angle;

use std::f32::consts::PI;

pub fn rot_x(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let x     = pos.0;
    let mut y = pos.1;
    let mut z = pos.2;

    let r = y.hypot(z);         // radius from axis
    let mut theta = z.atan2(y); // angle from plane
    theta += angle;                  // applying rotating

    y = r*theta.cos();
    z = r*theta.sin();
    return ( x, y, z );
}
pub fn rot_y(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let mut x = pos.0;
    let y     = pos.1;
    let mut z = pos.2;
        
    let r = x.hypot(z);         // radius from axis
    let mut theta = x.atan2(z); // angle from axis
    theta += angle;                  // applying rotating

    z = r*theta.cos();
    x = r*theta.sin();
    return ( x, y, z );
}
pub fn rot_z(pos: (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
    let mut x = pos.0;
    let mut y = pos.1;          // coordinates
    let z     = pos.2;
        
    let r = x.hypot(y);         // radius from axis
    let mut theta = y.atan2(x); // angle from axis
    theta += angle;                  // applying rotating
    
    x = r*theta.cos();
    y = r*theta.sin();               // calculating change
    return ( x, y, z );
}

pub struct Orbit {
    e: f32, // eccentricity                     0-1
    a: f32, // semimajor axis
        b: f32, // semiminor axis

    i: f32, // inclination                      0-pi
    o: f32, // longitude of the ascending node  0-2pi
    w: f32, // argument of periapsis            0-2pi

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
    pub fn pos(&self, M: f32) -> (f64, f64, f64) {
        let E = self.E(M);
        let mut pos = self.pos_elliptic(E);
        pos = rot_z(pos, self.w); // apply argument of periapsis
        pos = rot_x(pos, self.i); // apply inclination
        pos = rot_z(pos, self.o); // apply longitude of the ascending node
        return (pos.0 as f64, pos.1 as f64, pos.2 as f64);
    }
    fn pos_elliptic(&self, E: f32) -> ( f32, f32, f32 ) {
        // reference direction is +x
        // 'up' is +z
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
