use super::{PolyInfo, WIDTH, HEIGHT, SCALE, EPSILON, A, ITERATIONS, USE_SIMD};
use super::complex_simd::Complex8;
use core_simd::f64x8;
use num::complex::Complex;
// use super::polynomial::Polynomial;

pub fn calc_row(y: usize, table: &mut [usize], info: &PolyInfo, center: &Complex<f64>) {
  let mut x: usize = 0;
  if USE_SIMD {
      while x + 7 < WIDTH {
          let mut c = [Complex::new(0.0, 0.0); 8];
          for i in 0..8 {
              c[i] = Complex::new((x + i) as f64 - WIDTH as f64 / 2.0, y as f64 - HEIGHT as f64 / 2.0);
          }
          let mut c = Complex8::from(c) / WIDTH.max(HEIGHT) as f64 * 2.0 * SCALE + center;

          c = newton_raphson8(c, info);

          let c: [Complex<f64>; 8] = c.into();
          for dx in 0..8 {
              find_color(c[dx], x + dx, info, table);
          }

          x += 8;
      }

      if x > 7 {
          x -= 7; // we might have unfinished work to do
      }
  }

  while x < WIDTH {
      let mut c = Complex::new(x as f64 - WIDTH as f64 / 2.0, y as f64 - HEIGHT as f64 / 2.0) / (WIDTH.max(HEIGHT)) as f64 * 2.0 * SCALE + center;

      c = newton_raphson(c, info);

      find_color(c, x, info, table);

      x += 1;
  }
}

pub fn newton_raphson(mut c: Complex<f64>, info: &PolyInfo) -> Complex<f64> {
  for n in 0..ITERATIONS {
      c -= info.f.eval(c) / info.df.eval(c) * A;
      if n % 10 == 0 {
          for root in info.roots.iter() {
              if (c - root).norm() < EPSILON {
                  return c;
              }
          }
      }
  }

  c
}

pub fn newton_raphson8(mut c: Complex8, info: &PolyInfo) -> Complex8 {
  for n in 0..ITERATIONS {
      c -= info.f.eval8(c) / info.df.eval8(c) * A;
      if n % 10 == 0 {
          for root in info.roots.iter() {
              if (c - root).norm().lanes_lt(f64x8::splat(EPSILON)).all() {
                  return c;
              }
          }
      }
  }

  c
}

fn find_color(c: Complex<f64>, x: usize, info: &PolyInfo, table: &mut [usize]) {
    let mut color = info.roots.len();
    for i in 0..info.roots.len() {
        if (c - info.roots[i]).norm() < EPSILON {
            color = i;
        }
    }
    table[x] = color;
}
