use sdl2::{pixels::PixelFormatEnum, render::WindowCanvas};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: usize = 10;
const BG_COLOR: u8 = u8::MIN; // black background
const FG_COLOR: u8 = u8::MAX; // white foreground

pub struct Display {
    canvas: WindowCanvas,
    pixels: [bool; (WIDTH * SCALE) * (HEIGHT * SCALE)],
}

impl Display {
    pub fn new(canvas: WindowCanvas) -> Self {
        Self {
            canvas,
            pixels: [false; (WIDTH * SCALE) * (HEIGHT * SCALE)],
        }
    }

    /// Get the value of the pixel at the given coordinates.
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[x + WIDTH * y]
    }

    /// Toggle the pixel at the given coordinates.
    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        self.pixels[x + WIDTH * y] ^= true;
    }

    /// Clear the display.
    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|p| *p = false);
    }

    /// Render pixels onto the display.
    pub fn render(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_static(
                PixelFormatEnum::RGB332,
                (WIDTH * SCALE) as u32,
                (HEIGHT * SCALE) as u32,
            )
            .expect("unable to create texture");

        let pixel_data = self.pixels.map(|p| if p { FG_COLOR } else { BG_COLOR });

        texture
            .update(None, &pixel_data, WIDTH * std::mem::size_of::<u8>())
            .expect("unable to update texture");

        self.clear();
        self.canvas
            .copy(&texture, None, None)
            .expect("unable to copy texture to canvas");
        self.canvas.present();
    }
}
