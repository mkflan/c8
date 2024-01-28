use sdl2::{event::Event, keyboard::Scancode};

#[derive(Default)]
pub struct Keyboard([bool; 16]);

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    /// Press a key.
    pub fn press_key(&mut self, scancode: Scancode) {
        match scancode {
            Scancode::Num1 => self.0[0] = true,
            Scancode::Num2 => self.0[1] = true,
            Scancode::Num3 => self.0[2] = true,
            Scancode::Num4 => self.0[3] = true,
            Scancode::Q => self.0[4] = true,
            Scancode::W => self.0[5] = true,
            Scancode::E => self.0[6] = true,
            Scancode::R => self.0[7] = true,
            Scancode::A => self.0[8] = true,
            Scancode::S => self.0[9] = true,
            Scancode::D => self.0[10] = true,
            Scancode::F => self.0[11] = true,
            Scancode::Z => self.0[12] = true,
            Scancode::X => self.0[13] = true,
            Scancode::C => self.0[14] = true,
            Scancode::V => self.0[15] = true,
            _ => panic!("unrecognized key"),
        }
    }

    /// Release a key.
    pub fn release_key(&mut self, scancode: Scancode) {
        match scancode {
            Scancode::Num1 => self.0[0] = false,
            Scancode::Num2 => self.0[1] = false,
            Scancode::Num3 => self.0[2] = false,
            Scancode::Num4 => self.0[3] = false,
            Scancode::Q => self.0[4] = false,
            Scancode::W => self.0[5] = false,
            Scancode::E => self.0[6] = false,
            Scancode::R => self.0[7] = false,
            Scancode::A => self.0[8] = false,
            Scancode::S => self.0[9] = false,
            Scancode::D => self.0[10] = false,
            Scancode::F => self.0[11] = false,
            Scancode::Z => self.0[12] = false,
            Scancode::X => self.0[13] = false,
            Scancode::C => self.0[14] = false,
            Scancode::V => self.0[15] = false,
            _ => panic!("unrecognized key"),
        }
    }

    /// Check if a key is pressed.
    pub fn is_key_pressed(&self, idx: usize) -> bool {
        self.0[idx]
    }
}
