#![feature(portable_simd)]

extern crate image;
use image::{RgbImage, Rgb};
use num::complex::Complex;
use num::traits::FloatConst;

mod polynomial;
pub use polynomial::Polynomial;

mod complex_simd;

mod newton;
use newton::calc_row;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const ITERATIONS: usize = 1000;
const SCALE: f64 = 0.3;
const EPSILON: f64 = 0.02;
const A: f64 = 1.8;

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
    let center = Complex::new(1.414, 0.0);
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
    let mut table = vec![0; (WIDTH * HEIGHT) as usize];

    for y in 0..HEIGHT {
        if y % (HEIGHT / 100) == 0 {
            println!("{:.2}%", y as f32 / HEIGHT as f32 * 100.0);
        }
        calc_row(y, &mut table, &poly_info, &center);
    }

    println!("Drawing...");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = table[(x + y * WIDTH) as usize];

            if color == poly_info.roots.len() || is_edge(x, y, &table) {
                image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                let a = color as f64 / poly_info.roots.len() as f64 * 2.0 * f64::PI();
                let r = 200.0;
                let g = (a.sin() + 1.0) / 2.0 * 150.0 + 50.0;
                let b = (-a.cos() + 1.0) / 2.0 * 150.0 + 50.0;
                image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
            }
            // image.put_pixel(x, y, Rgb([(200.0 - re * 200.0) as u8, (200.0 - im * 200.0) as u8, 128u8]))
        }
    }

    image.save("output.png").unwrap();
}

fn is_edge(x: u32, y: u32, table: &[usize]) -> bool {
    let color = table[(x + y * WIDTH) as usize];
    if x > 0 {
        if table[(x - 1 + y * WIDTH) as usize] != color {
            return true;
        }
        if y > 0 {
            if table[(x - 1 - WIDTH + y * WIDTH) as usize] != color {
                return true;
            }
        }
        if y < HEIGHT - 1 {
            if table[(x - 1 + WIDTH + y * WIDTH) as usize] != color {
                return true;
            }
        }
    }
    if x < WIDTH - 1 {
        if table[(x + 1 + y * WIDTH) as usize] != color {
            return true;
        }
        if y > 0 {
            if table[(x + 1 - WIDTH + y * WIDTH) as usize] != color {
                return true;
            }
        }
        if y < HEIGHT - 1 {
            if table[(x + 1 + WIDTH + y * WIDTH) as usize] != color {
                return true;
            }
        }
    }
    if y > 0 {
        if table[(x - WIDTH + y * WIDTH) as usize] != color {
            return true;
        }
    }
    if y < HEIGHT - 1 {
        if table[(x + WIDTH + y * WIDTH) as usize] != color {
            return true;
        }
    }

    return false;
}
