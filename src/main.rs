#![feature(portable_simd)]

extern crate scoped_threadpool;
extern crate image;
extern crate core_simd;
extern crate distance_transform;

mod polynomial;
mod complex_simd;
mod newton;
mod edge;

use std::sync::{Arc, Mutex};
use image::{RgbImage, Rgb};
use num::complex::Complex;
use num::traits::FloatConst;
use scoped_threadpool::Pool;
use distance_transform::*;

pub use polynomial::Polynomial;
use newton::calc_row;
use edge::is_edge;

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 8000;
const ITERATIONS: usize = 1000;
const SCALE: f64 = 0.3;
const EPSILON: f64 = 0.02;
const A: f64 = 1.8;
const THREADS: u32 = 16;

pub const USE_SIMD: bool = false;

pub struct PolyInfo {
    pub f: Polynomial,
    pub df: Polynomial,
    pub roots: Vec<Complex<f64>>,
}

#[allow(dead_code)]
fn ring(length: usize) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let r = i as f64 / length as f64 * 2.0 * f64::PI();
        res.push(Complex::new(r.cos(), r.sin()));
    }
    res
}

#[allow(dead_code)]
fn spiral(length: usize) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let r = i as f64 / length as f64 * 2.0 * f64::PI();
        res.push(Complex::new(r.cos() * (i as f64 / (length - 1) as f64), r.sin() * (i as f64 / (length - 1) as f64)));
    }
    res
}

fn main() {
    let center = Complex::new(0.85, 0.0);
    let mut roots = ring(8).into_iter().chain(ring(8).into_iter().map(|x| 2.0 * x)).collect::<Vec<_>>();
    roots.push(Complex::new(0.0, 0.0));


    let f = Polynomial::from_roots(&roots);
    let df = f.diff();

    println!("f(x) = {}", f);

    let poly_info = PolyInfo {
        f,
        df,
        roots
    };

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    let table = vec![0; (WIDTH * HEIGHT) as usize];

    // Compute the actual fractal
    println!("Running the Newton-Raphson algorithm...");
    let mut pool = Pool::new(THREADS);
    let table = Mutex::new(table);
    pool.scoped(|scoped| {
        let table = Arc::new(&table);
        let poly_info = &poly_info;
        let center = &center;
        for y in 0..HEIGHT {
            let table = Arc::clone(&table);
            scoped.execute(move || {
                let mut local_table = vec![0; WIDTH as usize];
                calc_row(y, &mut local_table, poly_info, center);

                match table.lock() {
                    Ok(mut lock) => {
                        for x in 0..WIDTH {
                            lock[(x + y * WIDTH) as usize] = local_table[x as usize];
                        }
                    }
                    Err(e) => panic!("{}", e),
                }
            });
        }
    });
    let table = table.into_inner().unwrap();

    // Compute "edge" matrix
    println!("Computing edge matrix...");
    let mut edge = BoolGrid::new(WIDTH as usize, HEIGHT as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            edge.set(x as usize, y as usize, is_edge(x, y, &table));
        }
    }

    // let proximity = vec![0.0; (WIDTH * HEIGHT) as usize];
    println!("Computing proximity matrix...");
    let proximity = dt2d(&edge);

    println!("Drawing...");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = table[(x + y * WIDTH) as usize];

            if color == poly_info.roots.len() {
                image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                let a = color as f64 / poly_info.roots.len() as f64 * 2.0 * f64::PI();
                let p = *proximity.get(x as usize, y as usize).unwrap();
                let p = sigma(p.powf(0.5) / (p + 2.0).ln() / 8.0).powf(0.5);
                let r = 240.0 * p;
                let g = ((a.sin() + 1.0) / 2.0 * 150.0 + 80.0) * p;
                let b = ((-a.cos() + 1.0) / 2.0 * 150.0 + 80.0) * p;
                image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
            }
            // image.put_pixel(x, y, Rgb([(200.0 - re * 200.0) as u8, (200.0 - im * 200.0) as u8, 128u8]))
        }
    }

    image.save("output.png").unwrap();
}

fn sigma(x: f64) -> f64 {
    1.0 - (-x).exp()
}
