use super::{WIDTH, HEIGHT};
use distance_transform::GenericGrid;
use num::traits::FloatConst;

pub fn is_edge(x: usize, y: usize, table: &[usize]) -> bool {
    let color = table[x + y * WIDTH];
    if x > 0 {
        if table[x - 1 + y * WIDTH] != color {
            return true;
        }
        if y > 0 {
            if table[x - 1 - WIDTH + y * WIDTH] != color {
                return true;
            }
        }
        if y < HEIGHT - 1 {
            if table[x - 1 + WIDTH + y * WIDTH] != color {
                return true;
            }
        }
    }
    if x < WIDTH - 1 {
        if table[x + 1 + y * WIDTH] != color {
            return true;
        }
        if y > 0 {
            if table[x + 1 - WIDTH + y * WIDTH] != color {
                return true;
            }
        }
        if y < HEIGHT - 1 {
            if table[x + 1 + WIDTH + y * WIDTH] != color {
                return true;
            }
        }
    }
    if y > 0 {
        if table[x - WIDTH + y * WIDTH] != color {
            return true;
        }
    }
    if y < HEIGHT - 1 {
        if table[x + WIDTH + y * WIDTH] != color {
            return true;
        }
    }

    return false;
}

pub fn gaussian_blur(map: &mut GenericGrid<f64>, radius: u8) {
    // Compute kernel
    let mut kernel = vec![0.0; (2 * radius + 1) as usize];
    let sqrt_2pi: f64 = (f64::PI() * 2.0).sqrt();
    for dx in -(radius as isize)..=(radius as isize) {
        let radius = radius as f64;
        let index = (dx + radius as isize) as usize;
        let dx = dx as f64;
        kernel[index] = (-dx * dx / 2.0 / radius / radius).exp() / sqrt_2pi / radius;
    }

    // First pass
    let mut buffer: GenericGrid<f64> = GenericGrid::new(map.width(), map.height());
    for (x, y, _c) in map.iter() {
        let mut sigma = 0.0;
        let mut acc = 0.0;
        for dx in -(radius as isize)..=(radius as isize) {
            let index = (dx + radius as isize) as usize;
            if dx < 0 && x < (-dx) as usize || dx > 0 && x + dx as usize >= buffer.width() {
                continue;
            }
            acc += kernel[index];
            sigma += *map.get((x as isize + dx) as usize, y).unwrap() * kernel[index];
        }
        buffer.set(x, y, sigma / acc);
    }

    // Second pass
    for (x, y, _c) in buffer.iter() {
        let mut sigma = 0.0;
        let mut acc = 0.0;
        for dy in -(radius as isize)..=(radius as isize) {
            let index = (dy + radius as isize) as usize;
            if dy < 0 && y < (-dy) as usize || dy > 0 && y + dy as usize >= buffer.height() {
                continue;
            }
            acc += kernel[index];
            sigma += *buffer.get(x, (y as isize + dy) as usize).unwrap() * kernel[index];
        }
        map.set(x, y, sigma / acc);
    }
}
