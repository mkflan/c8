use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::WindowCanvas,
};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
pub const SCALE: u32 = 10;

pub struct Display {
    canvas: WindowCanvas,
    pixels: [u32; (WIDTH * SCALE) as usize * (HEIGHT * SCALE) as usize],
}

impl Display {
    pub fn new(canvas: WindowCanvas) -> Self {
        let pixels = [0; (WIDTH * SCALE) as usize * (HEIGHT * SCALE) as usize];

        let mut display = Self { canvas, pixels };

        display.clear();
        display.canvas.set_draw_color(Color::WHITE);

        display
    }

    /// Get the value of the pixel at the given coordinates.
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels[x + (WIDTH * SCALE) as usize * y]
    }

    /// Set the pixel at the given coordinates.
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.pixels[x + (WIDTH * SCALE) as usize * y] ^= 1;
    }

    /// Clear the display.
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Render pixels onto the display.
    pub fn render(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB888, WIDTH * SCALE, HEIGHT * SCALE)
            .unwrap();

        let pixel_data = self
            .pixels
            .iter()
            .flat_map(|n: &u32| n.to_be_bytes())
            .collect::<Vec<_>>();

        texture.update(
            None,
            &pixel_data,
            WIDTH as usize * std::mem::size_of::<u8>(),
        );

        self.canvas.copy(&texture, None, None);

        self.canvas.present();
    }
}
