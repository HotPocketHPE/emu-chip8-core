use std::iter::repeat;

use crate::config::CHIP8_CONFIG;

#[derive(Debug, Clone)]
pub struct DisplayData {
    pub width: usize,
    pub height: usize,
    backing_arr: Vec<bool>,
}

impl DisplayData {
    fn new(width: usize, height: usize) -> DisplayData {
        DisplayData {
            width,
            height,
            backing_arr: vec![false; width * height],
        }
    }

    pub fn new_64x32() -> DisplayData {
        DisplayData::new(64, 32)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.backing_arr[x + y * self.width]
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: bool) {
        self.backing_arr[x + y * self.width] = val
    }

    pub fn clear(&mut self) {
        self.backing_arr.iter_mut().for_each(|e| *e = false)
    }

    pub fn draw(&mut self, sprite: &[u8], mut x: usize, mut y: usize) -> bool {
        fn get_sprite_pixel(sprite: &[u8], sprite_x: usize, sprite_y: usize) -> bool {
            let byte = sprite[sprite_y];
            return (byte >> (7 - sprite_x)) & 1 != 0;
        }

        x = x % self.width;
        y = y % self.height;
        let mut collision = false;

        for y_offset in 0..sprite.len() {
            for x_offset in 0..8 {
                let pixel_val = get_sprite_pixel(sprite, x_offset, y_offset);
                let mut pixel_x = x + x_offset;
                let mut pixel_y = y + y_offset;

                if !CHIP8_CONFIG.sprite_clipping {
                    pixel_x = pixel_x % self.width;
                    pixel_y = pixel_y % self.height;
                }

                if pixel_x < self.width && pixel_y < self.height {
                    let old_data = self.get_pixel(pixel_x, pixel_y);
                    let new_data = old_data ^ pixel_val;
                    if old_data && !new_data {
                        collision = true;
                    }
                    self.set_pixel(pixel_x, pixel_y, new_data);
                }
            }
        }
        return collision;
    }

    pub fn debug_print(&self) {
        let line: String = repeat('_').take(self.width).collect();
        println!("{}", line);
        for j in 0..self.height {
            for i in 0..self.width {
                print!("{}", if self.get_pixel(i, j) { '#' } else { '.' });
            }
            print!("\n");
        }
        println!("{}", line);
    }
}
