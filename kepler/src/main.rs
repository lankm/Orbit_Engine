#![allow(unused)]
#![allow(non_snake_case)]


use std::{ops::{Add, Mul}, time::Instant, f32::consts::PI};


fn main() {
    graph();
    print_coords();
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

const COUNT: usize = 10;
fn graph() {
    use kepler::*;
    use plotters::prelude::*;
    // image format setup
    let area = BitMapBackend::gif(
        "images/animated.gif", 
        (256, 256), 
        80
    ).unwrap().into_drawing_area();
    
    // charting graph setup
    let mut chart = ChartBuilder::on(&area)
        .margin(0)
        .build_cartesian_3d(-5.0..5.0, -5.0..5.0, -5.0..5.0)
        .unwrap();

    //iterating
    for i in 0..=100 {
        // background
        area.fill(&WHITE).unwrap();

        // view setup
        chart.with_projection(|mut pb| {
            pb.pitch = 0.2;
            pb.yaw = 0.0314159265358979323*2.0*i as f64;
            pb.scale = 0.7;
            pb.into_matrix()
        });
        // orbit setup
        let orbit = Orbit::new(0.8, 5.0, PI/8.0, PI/2.0, PI/2.0, 0.0);
        const COUNT: usize = 100;
        const STEP: f32 = (2.0*PI/(COUNT as f32));
        // orbit render. the renderer has their axies messed up
        chart.draw_series(LineSeries::new(
        (0..COUNT+1).map(|M| {let c = orbit.pos(STEP*(M as f32)); (c.0, c.2, -c.1)}),
        &BLACK
        )).unwrap();
        // x y and z axes
        chart.draw_series(LineSeries::new(
            (-2..=10).map(|M| (M as f64, 0.0, 0.0)),
            &RED
        )).unwrap();
        chart.draw_series(LineSeries::new(
            (-2..=10).map(|M| (0.0, 0.0, -M as f64)),
            &GREEN
        )).unwrap();
        chart.draw_series(LineSeries::new(
            (-2..=10).map(|M| (0.0, M as f64, 0.0)),
            &BLUE
        )).unwrap();
        
        area.present().unwrap();
    }
}
fn print_coords() {
    use kepler::*;
    use std::f32::consts::PI;
    let orbit = Orbit::new(0.8, 5.0, 0.0, 0.0, PI/2.0, 0.0);

    const COUNT: i32 = 10;
    const STEP: f32 = (2.0*PI/(COUNT as f32));

    for i in 0..COUNT {
        let M = STEP*(i as f32);
        let pos = orbit.pos(M);
        println!("({}, {}, {})", pos.0, pos.1, pos.2);
    }
}