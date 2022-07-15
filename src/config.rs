#![allow(non_snake_case)]

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

use std::{path::Path, fs::write};

#[derive(Serialize, Deserialize)]
pub struct Chip8Config {
    pub shifting_with_Vy: bool,
    pub sprite_clipping: bool,
    pub emulate_draw_vblank_delay: bool,
    pub vblank_idle_cycles: u32,
}
 
impl Chip8Config {
    fn default() -> Chip8Config {
        Chip8Config { 
            shifting_with_Vy: true,
            sprite_clipping: true, 
            emulate_draw_vblank_delay: false,
            vblank_idle_cycles: 200,
        }
    }
}

lazy_static! {
    pub static ref CHIP8_CONFIG: Chip8Config = build_config();
}


fn build_config() -> Chip8Config {
    let path = Path::new("emu-chip8-core-config.json");
    if !path.exists() {
        write(path, serde_json::to_string(&Chip8Config::default()).unwrap())
        .expect("Couldn't create JSON config file:"); 
    }

    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
    .expect("Couldn't read JSON config:")
}