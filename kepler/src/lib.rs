#![allow(non_snake_case)]
#![allow(unused)]

pub struct Orbit {
    e: f32, // eccentricity
    a: f32, // semimajor axis
    b: f32, // semiminor axis

    i: f32, // inclination
    o: f32, // longitude of the ascending node
    w: f32, // argument of periapsis

    M0: f32, // initial mean anomoly
    t0: f32, // initial time
}
impl Orbit {
    pub fn new( e: f32, a: f32, i: f32, o: f32, w: f32 ) -> Orbit {
        return Orbit { 
            e, 
            a, 
            b: Orbit::b(e, a), 
            i, 
            o, 
            w,
            M0: 0.0,
            t0: 0.0,
        }
    }

    pub fn E(&self, M: f32) -> f32{
        let mut E: f32 = 0.0;

        for i in 0..1000 {
            E = M + self.e*E.sin();
        }

        return E;
    }
    pub fn pos(&self, M: f32) -> (f32, f32) {
        let E = self.E(M);
    
        let x = self.a*(E.cos()-self.e);
        let y = self.b*E.sin();
        return ( x, y );
    }
    
    fn a(e: f32, b: f32) -> f32{
        return b*( 1.0/(1.0-e*e) ).sqrt();
    }
    fn b(e: f32, a: f32) -> f32{
        return a*(1.0-e*e).sqrt();
    }
    fn e(a: f32, b: f32) -> f32{
        return (1.0-( (b*b)/(a*a) )).sqrt();
    }
    
}