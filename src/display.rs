

pub const FONT_LETTER_SIZE: usize = 5;
pub const FONT_DATA: [u8; FONT_LETTER_SIZE*16] = [
    //0
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    //1
    0x20, 0x60, 0x20, 0x20, 0x70,
    //2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    //3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    //4
    0x90, 0x90, 0xF0, 0x10, 0x10,
    //5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    //6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    //7
    0xF0, 0x10, 0x20, 0x40, 0x40,
    //8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    //9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    //A
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    //B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    //C
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    //D
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    //E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    //F
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

#[derive(Clone)]
pub struct DisplayData {
    pub width: usize,
    pub height: usize,
    backing_arr: Vec<bool>
}

impl DisplayData {
    fn new(width: usize, height: usize) -> DisplayData {
        DisplayData { width, height, backing_arr: vec![false; width * height] }
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

    pub fn draw(&mut self, sprite: &[u8], x: usize, y: usize) -> bool {
        let mut collision = false;

        let sp_iter: SpriteIter = sprite.into();
        for (x_offset, y_offset, pixel_val) in sp_iter {
            let pixel_x = (x + x_offset as usize) % self.width;
            let pixel_y = (y + y_offset as usize) % self.height;
            let old_data = self.get_pixel(pixel_x, pixel_y);
            let new_data = old_data ^ pixel_val;
            if old_data != new_data {
                collision = true;
            }
            self.set_pixel(pixel_x, pixel_y, new_data);
        }
        return collision;
    }
}

struct SpriteIter<'a> {
    sprite: &'a [u8],
    curr: usize
}

impl<'a> From<&'a [u8]> for SpriteIter<'a> {
    fn from(sprite: &'a [u8]) -> Self {
        Self { sprite, curr: 0 }
    }
}

impl<'a> Iterator for SpriteIter<'a> {
    type Item = (u8, u8, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr * 8 >= self.sprite.len() {
            return None;
        }
        let x_offset = (self.curr % 8) as u8;
        let y_offset = (self.curr / 8) as u8;
        let byte = self.sprite[y_offset as usize];
        let shift = 7 - (x_offset);
        let val = (byte >> shift) & 1 != 0;
        self.curr += 1;
        return Some((x_offset, y_offset, val));
    }
}