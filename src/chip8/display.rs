

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

pub struct DisplayData {
    data: Vec<Vec<bool>>,
    width: usize,
    height: usize
}

impl DisplayData {
    fn new(width: usize, height: usize) -> DisplayData {
        DisplayData { data: vec![vec![false; height]; width], width, height }
    }

    pub fn new_64x32() -> DisplayData {
        DisplayData::new(64, 32)
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
            let new_x = (x + x_offset) % self.width;
            let new_y = (y + y_offset) % self.height;
            let old_data = self.data[new_x][new_y];
            self.data[new_x][new_y] ^= sprite_as_bools[i];
            if self.data[new_x][new_y] != old_data {
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