

#[derive(Default)]
pub struct KeyboardState {
    pub key: [bool; 0x10],
    pub key_oldstate: [bool; 0x10],
    pub just_pressed: Option<u8>
}