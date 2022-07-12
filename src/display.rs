

const FONT_DATA: [u8; 5*16] = [
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
    pub data: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
    backing_arr: Vec<u32>
}

impl DisplayData {
    fn new(width: usize, height: usize) -> DisplayData {
        DisplayData { data: vec![vec![false; height]; width], width, height, backing_arr: vec![0; width * height] }
    }

    pub fn new_64x32() -> DisplayData {
        DisplayData::new(64, 32)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.backing_arr[x + y * self.width] > 0
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: bool) {
        self.backing_arr[x + y * self.width] = if val {1} else {0}
    }

    pub fn clear(&mut self) {
        for v in &mut self.data {
            for w in v {
                *w = false;
            }
        }
    }

    pub fn draw(&mut self, sprite: &[u8], x: usize, y: usize) -> bool {
        let mut collision = false;
        let sprite_as_bools = slice_u8_to_bool(sprite);
        for i in 0..sprite_as_bools.len() {
            let x_offset = i % 8;
            let y_offset = i / 8;
            let pixel_x = (x + x_offset) % self.width;
            let pixel_y = (y + y_offset) % self.height;
            let old_data = self.get_pixel(pixel_x, pixel_y);
            let new_data = old_data ^ sprite_as_bools[i];
            self.set_pixel(pixel_x, pixel_y, new_data);
            if self.get_pixel(pixel_x, pixel_y) != old_data {
                collision = true;
            }
        }
        return collision;
    }
}

fn slice_u8_to_bool(s: &[u8]) -> Vec<bool>{
    let mut v: Vec<bool> = Vec::with_capacity(s.len() * 8); 
    for b in s {
        for i in 7..=0 {
            v.push(((*b >> i) & 0b00000001) != 0);
        } 
    }
    return v;
}