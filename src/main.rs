extern crate image;
use image::{RgbImage, Rgb};
use num::complex::Complex;
use num::traits::FloatConst;

mod polynomial;
pub use polynomial::Polynomial;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const ITERATIONS: usize = 1000;
const SCALE: f64 = 3.3;
const EPSILON: f64 = 0.02;
const A: f64 = 1.8;

fn ring(length: usize) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let r = i as f64 / length as f64 * 2.0 * f64::PI();
        res.push(Complex::new(r.cos(), r.sin()));
    }
    res
}

fn spiral(length: usize) -> Vec<Complex<f64>> {
    let mut res = Vec::with_capacity(length);
    for i in 0..length {
        let r = i as f64 / length as f64 * 2.0 * f64::PI();
        res.push(Complex::new(r.cos() * (i as f64 / (length - 1) as f64), r.sin() * (i as f64 / (length - 1) as f64)));
    }
    res
}

fn main() {
    // let center = Complex::new(1.414, 0.0);
    let center = Complex::new(0.0, 0.0);
    let mut roots = ring(8).into_iter().chain(ring(8).into_iter().map(|x| 2.0 * x)).collect::<Vec<_>>();
    roots.push(Complex::new(0.0, 0.0));
    // let roots = spiral(8).into_iter().chain(spiral(8).into_iter().map(|x| -x)).collect::<Vec<_>>();
    // let roots = vec![
    //     Complex::new(1.0, 0.0),
    //     Complex::new(0.0, 1.0),
    //     Complex::new(-1.0, 0.0),
    //     Complex::new(0.0, -1.0),
    //     Complex::new(1.0, 1.0),
    //     Complex::new(1.0, -1.0),
    //     Complex::new(-1.0, 1.0),
    //     Complex::new(-1.0, -1.0),
    //     Complex::new(0.0, 0.0),
        // Complex::new(-3.0, 1.0),
        // Complex::new(-2.0, -1.0),
        // Complex::new(-1.0, 1.0),
        // Complex::new(0.0, -1.0),
        // Complex::new(1.0, 1.0),
        // Complex::new(2.0, -1.0),
        // Complex::new(3.0, 1.0),
    // ];
    println!("{:?}", roots);
    let f = Polynomial::from_roots(&roots);
    let df = f.diff();

    println!("{}", f);

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    let mut table = vec![0; (WIDTH * HEIGHT) as usize];
    let mut steps = vec![0; (WIDTH * HEIGHT) as usize];

    for y in 0..HEIGHT {
        if y % (HEIGHT / 100) == 0 {
            println!("{:.2}%", y as f32 / HEIGHT as f32 * 100.0);
        }
        for x in 0..WIDTH {
            let mut c = (Complex::new(x as f64 - WIDTH as f64 / 2.0, y as f64 - HEIGHT as f64 / 2.0) / (WIDTH.max(HEIGHT)) as f64 + center) * 2.0 * SCALE;

            'l3: for n in 0..ITERATIONS {
                c -= f.eval(c) / df.eval(c) * A;
                // if n % 10 == 0 {
                    for root in roots.iter() {
                        if (c - root).norm() < EPSILON {
                            steps[(x + y * WIDTH) as usize] = n;
                            break 'l3;
                        }
                    }
                // }
            }
            if steps[(x + y * WIDTH) as usize] == 0 {
                steps[(x + y * WIDTH) as usize] = ITERATIONS;
            }

            let mut color = roots.len();
            for i in 0..roots.len() {
                if (c - roots[i]).norm() < EPSILON {
                    color = i;
                }
            }
            table[(x + y * WIDTH) as usize] = color;
        }
    }

    println!("Drawing...");

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = table[(x + y * WIDTH) as usize];
            let mut inside = 0;

            if x > 0 {
                if table[(x - 1 + y * WIDTH) as usize] != color {
                    inside += 1;
                }
                if y > 0 {
                    if table[(x - 1 - WIDTH + y * WIDTH) as usize] != color {
                        inside += 1;
                    }
                }
                if y < HEIGHT - 1 {
                    if table[(x - 1 + WIDTH + y * WIDTH) as usize] != color {
                        inside += 1;
                    }
                }
            }
            if x < WIDTH - 1 {
                if table[(x + 1 + y * WIDTH) as usize] != color {
                    inside += 1;
                }
                if y > 0 {
                    if table[(x + 1 - WIDTH + y * WIDTH) as usize] != color {
                        inside += 1;
                    }
                }
                if y < HEIGHT - 1 {
                    if table[(x + 1 + WIDTH + y * WIDTH) as usize] != color {
                        inside += 1;
                    }
                }
            }
            if y > 0 {
                if table[(x - WIDTH + y * WIDTH) as usize] != color {
                    inside += 1;
                }
            }
            if y < HEIGHT - 1 {
                if table[(x + WIDTH + y * WIDTH) as usize] != color {
                    inside += 1;
                }
            }

            if color == roots.len() || inside > 1 {
                image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                let a = color as f64 / roots.len() as f64 * 2.0 * f64::PI();
                let mut r = 200.0;
                let mut g = (a.sin() + 1.0) / 2.0 * 150.0 + 50.0;
                let mut b = (-a.cos() + 1.0) / 2.0 * 150.0 + 50.0;
                let s = steps[(x + WIDTH * y) as usize] as f64 / ITERATIONS as f64;
                // r *= 1.0 - s.sqrt();
                // g *= 1.0 - s.sqrt();
                // b *= 1.0 - s.sqrt();
                image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
            }
            // image.put_pixel(x, y, Rgb([(200.0 - re * 200.0) as u8, (200.0 - im * 200.0) as u8, 128u8]))
        }
    }

    image.save("output.png").unwrap();
}
