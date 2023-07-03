#![allow(unused)]
#![allow(non_snake_case)]


use std::{ops::{Add, Mul}, time::Instant, f64::consts::PI};
use kepler::*;


fn main() {
    let orbit = Orbit::new(1.5, 5.0, 0.0, 0.0, 0.0, 0.0);
    // graph(&orbit, 100, 200, 24);
    const COUNT: u32 = 100;
    print_coords(&orbit, COUNT);
}
fn pi_test() {
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

fn graph(orbit: &Orbit, count: u32, px: u32, fps: u32) {
    use kepler::*;
    use plotters::prelude::*;
    // image format setup
    let area = BitMapBackend::gif(
        "images/animated.gif", 
        (px, px), 
        1000/fps
    ).unwrap().into_drawing_area();
    
    // charting graph setup
    let size = orbit.a;
    let mut chart = ChartBuilder::on(&area)
        .margin(0)
        .build_cartesian_3d(-size..size, -size..size, -size..size)
        .unwrap();

    //iterating
    for i in 0..=100 {
        // background
        area.fill(&WHITE).unwrap();

        // view setup
        chart.with_projection(|mut pb| {
            pb.pitch = 0.2;
            pb.yaw = 2.0*PI/100.0*i as f64; //rotate and seemlessly loop
            pb.scale = 0.7;
            pb.into_matrix()
        });

        // orbit render. the renderer has their axies messed up
        let step: f64 = (2.0*PI/(count as f64));
        chart.draw_series(LineSeries::new(
        (0..count+1).map(|M| {let c = orbit.pos(step*(M as f64)); (c.0, c.2, -c.1)}),
        &BLACK
        )).unwrap();

        // x y and z axes
        chart.draw_series(LineSeries::new(
            (-1..=2).map(|M| (size*M as f64, 0.0, 0.0)),
            &RED
        )).unwrap();
        chart.draw_series(LineSeries::new(
            (-1..=2).map(|M| (0.0, 0.0, -size*M as f64)),
            &GREEN
        )).unwrap();
        chart.draw_series(LineSeries::new(
            (-1..=2).map(|M| (0.0, size*M as f64, 0.0)),
            &BLUE
        )).unwrap();
        
        // print to gif
        area.present().unwrap();
    }
}
fn print_coords(orbit: &Orbit, count: u32) {
    let step: f64 = (4.0*PI/(count as f64));

    for i in 0..count {
        let M = step*(i as f64);
        
        let pos = orbit.pos(M);
        // println!("({}, {})", pos.0, pos.1);
    }
}