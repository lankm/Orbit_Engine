pub fn calc_E(M: f32, e: f32) -> f32 {
    let mut E: f32 = 0.0;

    for i in 0..1000 {
        E = M + e*E.sin();
    }

    return E;
}
pub fn calc_e(a: f32, b: f32) -> f32 {
    let e = (1.0-((b*b)/(a*a))).sqrt();
    return e;
}
pub fn calc_pos(M: f32, a: f32, b: f32) -> (f32, f32) {
    let e = calc_e(a, b);
    let E = calc_E(M, e);

    let x = a*(E.cos()-e);
    let y = b*E.sin();
    return ( x, y );
}