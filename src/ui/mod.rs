mod display_ui;
pub mod update_keypress;

use std::time::Duration;

use eframe::egui;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

use self::display_ui::{build_display_texture};


#[derive(Default)]
pub struct EguiApp {

}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CHIP-8 Emulator");
        });
    }
}

pub fn run_sdl() {
    let width = 640;
    let height = 480;

    let sdl_context = sdl2::init().unwrap();
    let sdl_video_subsystem = sdl_context.video().unwrap();

    let window = sdl_video_subsystem.window("CHIP-8 Emulator", width, height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
 
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        build_display_texture(canvas.texture_creator());
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
    
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }