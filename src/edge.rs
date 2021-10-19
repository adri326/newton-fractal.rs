use super::{WIDTH, HEIGHT};

pub fn is_edge(x: u32, y: u32, table: &[usize]) -> bool {
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
