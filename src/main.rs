use ui::{EguiApp, run_sdl};

mod chip8;
mod ui;

fn main() {
    //let native_options = eframe::NativeOptions::default();
    //eframe::run_native("CHIP-8 Emulator", native_options, Box::new(|cc| Box::new(EguiApp::new(cc))));

    run_sdl();
}
