use sdl2::{render::TextureCreator, video::WindowContext, surface::Surface};



pub fn build_display_texture(tc: TextureCreator<WindowContext>) {
    
}

fn build_pixel_data<'a>(data_bools: &[&[bool]], buffer: &'a mut [u8]) -> Surface<'a> {
    let width = data_bools.len();
    let height = data_bools[0].len();
    assert!(buffer.len() == width * height);
    for i in 0..width {
        for j in 0..height {
            buffer[i * width + j] = (if data_bools[i][j] {0b10000000} else {0});
        }
    }
    return Surface::from_data(buffer, data_bools.len() as u32, data_bools[0].len() as u32, 8, 
        sdl2::pixels::PixelFormatEnum::Index1MSB).unwrap();
}
