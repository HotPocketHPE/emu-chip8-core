#![allow(non_snake_case)]

pub struct KeyboardState {
    pub key: [bool; 0x10],
    pub Fx0A: Fx0AStatus,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            key: [false; 0x10],
            Fx0A: Fx0AStatus::Inactive,
        }
    }

    pub fn press_key(&mut self, key: u8) {
        let just_pressed = !self.key[key as usize];
        self.key[key as usize] = true;
        if let Fx0AStatus::WaitingForPress = self.Fx0A {
            if just_pressed {
                self.Fx0A = Fx0AStatus::WaitingForRelease(key);
            }
        }
    }

    pub fn release_key(&mut self, key: u8) {
        self.key[key as usize] = false;
        if let Fx0AStatus::WaitingForRelease(k) = self.Fx0A {
            if k == key {
                self.Fx0A = Fx0AStatus::JustReleased(key);
            }
        }
    }
}

pub enum Fx0AStatus {
    Inactive,
    WaitingForPress,
    WaitingForRelease(u8),
    JustReleased(u8),
}
