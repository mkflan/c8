use sdl2::{pixels::PixelFormatEnum, render::WindowCanvas};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: usize = 10;
const BG_COLOR: u32 = u32::MIN; // black background
const FG_COLOR: u32 = u32::MAX; // white foreground

pub struct Display {
    canvas: WindowCanvas,
    pixels: [u32; (WIDTH * SCALE) * (HEIGHT * SCALE)],
}

impl Display {
    pub fn new(canvas: WindowCanvas) -> Self {
        let pixels = [BG_COLOR; (WIDTH * SCALE) * (HEIGHT * SCALE)];

        Self { canvas, pixels }
    }

    /// Get the value of the pixel at the given coordinates.
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels[x + (WIDTH * SCALE) * y]
    }

    /// Set the pixel at the given coordinates.
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.pixels[x + (WIDTH * SCALE) * y] ^= 1;
    }

    /// Clear the display.
    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|p| *p = BG_COLOR);
    }

    /// Render pixels onto the display.
    pub fn render(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_static(
                PixelFormatEnum::BGRA8888,
                (WIDTH * SCALE) as u32,
                (HEIGHT * SCALE) as u32,
            )
            .expect("unable to create texture");

        let pixel_data = self
            .pixels
            .iter()
            .flat_map(|n: &u32| n.to_be_bytes())
            .collect::<Vec<_>>();

        texture
            .update(
                None,
                &pixel_data,
                WIDTH as usize * std::mem::size_of::<u8>(),
            )
            .expect("unable to update texture");

        self.clear();
        self.canvas
            .copy(&texture, None, None)
            .expect("unable to copy texture to canvas");
        self.canvas.present();
    }
}
