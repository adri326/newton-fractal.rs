#![feature(portable_simd)]

extern crate scoped_threadpool;
extern crate image;
extern crate core_simd;
extern crate distance_transform;

mod polynomial;
mod complex_simd;
mod newton;
mod draw;

use std::sync::{Arc, Mutex};
use image::{RgbImage, Rgb};
use num::complex::Complex;
use num::traits::FloatConst;
use scoped_threadpool::Pool;
use distance_transform::*;

pub use polynomial::Polynomial;
use newton::calc_row;
use draw::{is_edge, gaussian_blur};

const WIDTH: usize = 1080 * 3;
const HEIGHT: usize = 1350 * 3;
const ITERATIONS: usize = 1000;
const SCALE: f64 = 36.0;
const EPSILON: f64 = 0.02;
const A: f64 = 1.95;
const THREADS: u32 = 16;
const FRAMES: usize = 800;
const SHADOW: (f64, f64) = (0.2, 0.9);
const SHADOW_STRENGTH: f64 = 1.0;

pub const USE_SIMD: bool = false;

pub struct PolyInfo {
    pub f: Polynomial,
    pub df: Polynomial,
    pub roots: Vec<Complex<f64>>,
}

fn main() {
    let center = Complex::new(0.0, 0.0);
    // fractal(0, Complex::new(0.0, 0.0));
    // fractal(0, Complex::new(1.8, 0.0));
    for frame in 0..FRAMES {
        fractal(frame, center);
    }
}

#[allow(dead_code)]
fn ring(length: usize) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let a = i as f64 / length as f64 * 2.0 * f64::PI();
        res.push(Complex::new(a.cos(), a.sin()));
    }
    res
}

#[allow(dead_code)]
fn ringoid(length: usize, current_length: f64) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let a = i as f64 / current_length * 2.0 * f64::PI();
        let r = (8.0 * (i as f64 - current_length) + 2.0).exp() + 1.0;
        res.push(Complex::new(a.cos() * r, a.sin() * r));
    }
    res
}

#[allow(dead_code)]
fn spiral(length: usize, angle: f64, coeff: f64) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let r = i as f64 * angle;
        res.push(Complex::new(r.cos() * coeff.powf(i as f64), r.sin() * coeff.powf(i as f64)));
    }
    res
}

fn fractal(frame: usize, center: Complex<f64>) {
    println!("Begin frame {}", frame);
    let frame_ratio = frame as f64 / FRAMES as f64;
    let frame_ratio = -(frame_ratio * f64::PI()).cos() * 0.5 + 0.5;
    let shadow = normalize(SHADOW);
    let mut roots = ringoid(8, frame_ratio * 7.0 + 2.0).into_iter().map(|x| Complex::new((0.02 * frame_ratio).cos(), (0.02 * frame_ratio).sin()) * x).collect::<Vec<_>>();
    // let mut roots = ring(8).into_iter().chain(ring(8).into_iter().map(|x| 2.0 * x)).collect::<Vec<_>>();

    roots.push(Complex::new(0.0, 0.0));

    let bg_color_id = roots.len() - 1;
    let f = Polynomial::from_roots(&roots);
    let df = f.diff();

    // println!("f(x) = {}", f);

    let poly_info = PolyInfo {
        f,
        df,
        roots
    };

    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    let table = vec![0; WIDTH * HEIGHT];

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
                let mut local_table = vec![0; WIDTH];
                calc_row(y, &mut local_table, poly_info, center);

                match table.lock() {
                    Ok(mut lock) => {
                        for x in 0..WIDTH {
                            lock[(x + y * WIDTH) as usize] = local_table[x];
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
    let mut edge = BoolGrid::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            edge.set(x, y, is_edge(x, y, &table));
        }
    }

    println!("Computing proximity matrix...");
    let mut proximity = dt2d(&edge);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = *proximity.get(x, y).unwrap();
            proximity.set(x, y, sigma(p.powf(0.55) / (p + 2.0).ln() / 16.0).powf(0.5));
        }
    }

    println!("Computing shadow...");
    let mut nabla_map_dx = GenericGrid::new(WIDTH, HEIGHT);
    let mut nabla_map_dy = GenericGrid::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let nabla = normalize(discrete_nabla(&proximity, x, y).unwrap());
            nabla_map_dx.set(x, y, nabla.0);
            nabla_map_dy.set(x, y, nabla.1);
        }
    }

    gaussian_blur(&mut nabla_map_dx, 4);
    gaussian_blur(&mut nabla_map_dy, 4);

    println!("Drawing...");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = table[(x + y * WIDTH) as usize];

            if color == poly_info.roots.len() {
                image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
            } else {
                let a = color as f64 / poly_info.roots.len() as f64 * 2.0 * f64::PI() - 0.5;
                let nabla = (*nabla_map_dx.get(x, y).unwrap(), *nabla_map_dy.get(x, y).unwrap());
                let shadow_orient = if color == bg_color_id { 1.0 } else { 0.3 };
                let s = 1.0 - (1.0 - *proximity.get(x, y).unwrap()) * (1.0 - (shadow_orient * point_mul(nabla, shadow)).max(0.0) * SHADOW_STRENGTH);

                let (r, g, b) = if color == bg_color_id {
                    let r = 50.0;
                    let g = 50.0;
                    let b = 55.0;
                    (r * s, g * s, b * s)
                } else {
                    let s = 0.5 * s + 0.5;
                    let r = 250.0;
                    let g = (a.sin() + 1.0) / 2.0 * 160.0 + 70.0;
                    let b = (-a.cos() + 1.0) / 2.0 * 160.0 + 70.0;
                    (r * s, g * s, b * s)
                };
                image.put_pixel(x as u32, y as u32, Rgb([r as u8, g as u8, b as u8]));
            }
            // image.put_pixel(x, y, Rgb([(200.0 - re * 200.0) as u8, (200.0 - im * 200.0) as u8, 128u8]))
        }
    }

    image.save(format!("output/{}.png", frame)).unwrap();
}

fn sigma(x: f64) -> f64 {
    1.0 - (-x).exp()
}

fn discrete_nabla(field: &GenericGrid<f64>, x: usize, y: usize) -> Option<(f64, f64)> {
    let dx = if x == 0 {
        field.get(x + 1, y)? - field.get(x, y)?
    } else if x == field.width() - 1 {
        field.get(x, y)? - field.get(x - 1, y)?
    } else {
        (field.get(x + 1, y)? - field.get(x - 1, y)?) / 2.0
    };

    let dy = if y == 0 {
        field.get(x, y + 1)? - field.get(x, y)?
    } else if y == field.height() - 1 {
        field.get(x, y)? - field.get(x, y - 1)?
    } else {
        (field.get(x, y + 1)? - field.get(x, y - 1)?) / 2.0
    };

    Some((dx, dy))
}

#[inline]
fn normalize((x, y): (f64, f64)) -> (f64, f64) {
    if x == 0.0 && y == 0.0 {
        (0.0, 0.0)
    } else {
        let d = (x * x + y * y).sqrt();
        (x / d, y / d)
    }
}

#[inline]
fn point_mul((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    x1 * x2 + y1 * y2
}
