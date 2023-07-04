/* README
 * This is a set of orbital parameters. Currently everything is floats, but
 * This should change to fixed point numbers due to the loss of precision.
 * Space has no center so it should not be more accurate depending on the time
 * or location.
 */
#![allow(non_snake_case)]
#![allow(unused)]

pub mod angle;

use std::{f64::consts::PI, char::MAX};

pub fn rot_x(pos: (f64, f64, f64), angle: f64) -> (f64, f64, f64) {
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
pub fn rot_y(pos: (f64, f64, f64), angle: f64) -> (f64, f64, f64) {
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
pub fn rot_z(pos: (f64, f64, f64), angle: f64) -> (f64, f64, f64) {
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
    pub e: f64, // eccentricity                     0-1
    pub a: f64, // semimajor axis
    pub b: f64, // semiminor axis

    pub i: f64, // inclination                      0-pi
    pub o: f64, // longitude of the ascending node  0-2pi
    pub w: f64, // argument of periapsis            0-2pi

    pub t0: f64, // time of periapsis passage
}
impl Orbit {
    // TODO: change a/b to periapsis. then calculate a/b
    pub fn new( mut e: f64, periapsis: f64, i: f64, o: f64, w: f64, t0: f64 ) -> Orbit {
        e = e.abs(); // safety feature
        let a = Orbit::a_from_periapsis(periapsis, e);
        return Orbit { 
            e, 
            a,
            b: Orbit::b_from_a(a, e), 
            i, 
            o, 
            w,
            t0,
        }
    }

    /* x+ is the reference direction
     * y+ is theta+
     * z+ is 'north'
     */
    pub fn pos(&self, M: f64) -> (f64, f64, f64) {
        let mut pos = if self.e < 1.0 { // if elliptic
            let E = self.E(M);
            self.pos_elliptic(E)
        } else {                                         // if hyperbolic
            let H = self.H(M);
            self.pos_hyperbolic(H)
        };
        
        pos = rot_z(pos, self.w); // apply argument of periapsis
        pos = rot_x(pos, self.i); // apply inclination
        pos = rot_z(pos, self.o); // apply longitude of the ascending node

        return (pos.0 as f64, pos.1 as f64, pos.2 as f64);
    }
    /* E calculation
     * Newton-Raphson method, but made it stable. Getting the remainder of the
     * result is required due to the function being non-continuous.
     * Very slowly starts to break if e=1 and M=0.
     * 
     * max average steps: 4.87125 (e = 1, M = 0-2PI)
     * min average steps: 0.00000 (e = 0)
     * max steps needed:  N/A     (e = 1, M = 0)
     * min steps needed:  0       (e = 0)
     */
    fn E(&self, M: f64) -> f64 {
        const PRECISION: f64 = 9e-16;   // min stable number
        const MAX_ITER: u32 = 100;      // if e = ~1 and M = ~0
        let mut E: f64 = M;  // initial estimate

        for i in 0..MAX_ITER {
            let E_next = M + self.e*E.sin(); // calculate next guess
            let E_diff = E_next - E;
            
            if E_diff.abs() < PRECISION {
                return E_next;
            } else {
                let E_prime = 1.0 - self.e*E.cos();
                E = E + ( E_diff/E_prime ) % 1.4; // 1.4 causes the best results
            }
        }

        return E;
    }
    fn pos_elliptic(&self, E: f64) -> ( f64, f64, f64 ) {
        let x = self.a*(E.cos()-self.e);
        let y = self.b*E.sin();

        return ( x, y, 0.0 );
    }
    /* H calculation
     * Newton-Raphson method. Due to hyperbolic functions being exponential, the calculations
     * are more complex and are reordered but still the same. Generally doesn't take more than
     * 10 iterations.
     * Only breaks if you travel faster than light next to a black hole.
     */
    fn H(&self, M: f64) -> f64 {
        if M == 0.0 { return 0.0 }

        const PRECISION: f64 = 4e-15;   // min stable number
        const MAX_ITER: u32 = 100;      // for safety
        let mut H: f64 = ((M.abs()+(6.0*M.abs()).powf(0.25))/self.e).asinh();             // initial estimate
        H = if M.is_sign_negative() {-H} else {H};

        let mut iter = MAX_ITER;
        for i in 0..MAX_ITER {
            // let H_next = -M + self.e*H.sinh(); // calculate next guess. H.sinh() can be inf
            let p1 = 1.0/(1.0/H.tanh() - 1.0/(self.e*H.sinh())); // done to avoid inf/inf
            let p2 = M/(self.e*H.cosh() - 1.0);
            let p3 = H/(self.e*H.cosh() - 1.0);
            let H_next = H - p1 + p2 + p3;
            let H_diff = H_next - H;
            
            if H_diff.abs() < PRECISION {
                return H_next;
            } else {
                // let H_prime = -1.0 + self.e*H.cosh(); // H.cosh() can be inf
                // H = H - ( H_diff/H_prime );
                H = H_next;
            }
            
        }
        return H;
    }
    fn pos_hyperbolic(&self, H: f64) -> ( f64, f64, f64 ) {
        let x = self.a*(H.cosh()-self.e);
        let y = self.b*H.sinh();

        return ( x, y, 0.0 );
    }

    fn a_from_periapsis(periapsis: f64, e: f64) -> f64 {
        return periapsis/(1.0-e);
    }
    fn b_from_a(a: f64, e: f64) -> f64 {
        return a*(1.0-e*e).abs().sqrt();
    }

    fn a(e: f64, b: f64) -> f64 {
        return b*( 1.0/(1.0-e*e) ).sqrt();
    }
    fn b(e: f64, a: f64) -> f64 {
        return a*(1.0-e*e).sqrt();
    }
    fn e(a: f64, b: f64) -> f64 {
        return (1.0-( (b*b)/(a*a) )).sqrt();
    }
    
    pub fn apoapsis(&self) -> f64 {
        return self.a - self.periapsis();
    }
    pub fn periapsis(&self) -> f64 {
        return ( self.a - self.a*self.e )/2.0
    }
}

pub struct Stat {
    pub total: f64,
    pub count: u64,
    pub max: f64,
    pub min: f64,
}
impl Stat {
    pub fn new() -> Stat {
        return Stat { total: 0.0, count: 0, max: f64::MIN, min : f64::MAX };
    }
    pub fn entry(&mut self, val: f64, count: Option<u64>) {
        self.total += val;
        self.count += count.unwrap_or(1);
        self.max = if val > self.max {val} else {self.max};
        self.min = if val < self.min {val} else {self.min};
    }
    pub fn mean(&self) -> f64 {
        return self.total / self.count as f64;
    } 
}