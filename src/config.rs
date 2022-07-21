#![allow(non_snake_case)]

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use std::{fs::write, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Chip8Config {
    pub clock_speed_hz: u64,
    pub shifting_with_Vy: bool,
    pub sprite_clipping: bool,
    pub emulate_draw_vblank_delay: bool,
}

impl Chip8Config {
    fn default() -> Chip8Config {
        Chip8Config {
            clock_speed_hz: 500,
            shifting_with_Vy: true,
            sprite_clipping: true,
            emulate_draw_vblank_delay: false,
        }
    }
}

lazy_static! {
    pub static ref CHIP8_CONFIG: Chip8Config = build_config();
}

fn build_config() -> Chip8Config {
    let path = Path::new("emu-chip8-core-config.json");
    if !path.exists() {
        write(
            path,
            serde_json::to_string(&Chip8Config::default()).unwrap(),
        )
        .expect("Couldn't create JSON config file:");
    }

    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
        .expect("Couldn't read JSON config:")
}
