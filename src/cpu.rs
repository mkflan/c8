use crate::{
    display::{Display, HEIGHT, SCALE, WIDTH},
    keyboard::Keyboard,
};
use arrayvec::ArrayVec;
use sdl2::{event::Event, keyboard::Scancode, EventPump};

/// The commonly used font.
const FONT: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// The memory address at which the font conventionally starts.
const FONT_START: usize = 0x050;

/// The memory address at which programs start.
const PROG_START: usize = 0x200;

/// The size of memory in bytes.
const MEM_SIZE: usize = 0x1000;

/// CPU state.
pub struct Cpu {
    /// Accessible memory (4096 bytes).
    mem: [u8; MEM_SIZE],

    /// Program counter.
    pc: u16,

    /// Index register (I).
    idxr: u16,

    /// The 16 8-bit general-purpose variable registers.
    gpvr: [u8; 16],

    /// The stack.
    stack: ArrayVec<u16, 16>,

    /// The delay timer register.
    dtr: u8,

    /// The sound timer register.
    str: u8,

    /// The display.
    display: Display,

    /// The keyboard.
    keyboard: Keyboard,

    /// The SDL event pump.
    event_pump: EventPump,

    /// A flag controlling whether the display should be rerendered after an instruction.
    rerender: bool,
}

impl Cpu {
    /// Create a new CPU initialized with default values.
    pub fn new(display: Display, event_pump: EventPump) -> Self {
        Self {
            mem: std::array::from_fn(|_| 0),
            pc: 0,
            idxr: 0,
            gpvr: std::array::from_fn(|_| 0),
            stack: ArrayVec::new(),
            dtr: 0,
            str: 0,
            display,
            event_pump,
            keyboard: Keyboard::new(),
            rerender: false,
        }
    }

    /// Read a byte from memory.
    fn read_byte(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    /// Write a byte to memory.
    fn write_byte(&mut self, addr: usize, byte: u8) {
        self.mem[addr] = byte;
    }

    /// Read a word from memory.
    fn read_word(&self, addr: usize) -> u16 {
        let hi = self.read_byte(addr) as u16;
        let lo = self.read_byte(addr + 1) as u16;

        hi << 8 | lo
    }

    /// Get the value of a register.
    fn get_reg(&self, register: usize) -> u8 {
        self.gpvr[register]
    }

    /// Set the value of a register.
    fn set_reg(&mut self, register: usize, val: u8) {
        self.gpvr[register] = val;
    }

    /// Push to the stack.
    fn push_stack(&mut self) {
        self.stack.push(self.pc);
    }

    /// Pop from the stack.
    fn pop_stack(&mut self) {
        self.pc = self.stack.pop().expect("no return address");
    }

    /// Skip the next instruction if the given condition is true.
    fn skip_inst_if(&mut self, cond: bool) {
        cond.then(|| self.pc += 2);
    }

    /// Implementation of the shift right instruction.
    fn inst_shiftr(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);

        if cfg!(feature = "modern-shift") {
            let lsb = regx_val & 1;
            self.set_reg(regx, regx_val >> 1);
            *self.gpvr.last_mut().unwrap() = lsb;
        } else {
            self.set_reg(regx, regy_val);
            let lsb = regy_val & 1;
            self.set_reg(regx, regy_val >> 1);
            *self.gpvr.last_mut().unwrap() = lsb;
        }
    }

    /// Implementation of the shift left instruction.
    fn inst_shiftl(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);

        if cfg!(feature = "modern-shift") {
            let msb = regx_val >> 7;
            self.set_reg(regx, regx_val << 1);
            *self.gpvr.last_mut().unwrap() = msb;
        } else {
            self.set_reg(regx, regy_val);
            let msb = regy_val >> 7;
            self.set_reg(regx, regy_val << 1);
            *self.gpvr.last_mut().unwrap() = msb;
        }
    }

    /// Fetch the next instruction.
    fn next_inst(&mut self) -> u16 {
        let inst = self.read_word(self.pc as usize);
        self.pc += 2;
        inst
    }

    /// Load a program into memory and prepare for execution.
    fn load_program(&mut self, prog: &[u8]) {
        // Load the font into memory.
        for (idx, &byte) in FONT.iter().enumerate() {
            self.mem[FONT_START + idx] = byte;
        }

        // Load the program into memory.
        for (idx, &byte) in prog.iter().enumerate() {
            self.mem[PROG_START + idx] = byte;
        }

        self.pc = PROG_START as u16;
    }

    /// Decode and execute an instruction.
    fn execute_instruction(&mut self, inst: u16) {
        self.rerender = false;

        println!("inst: {:#X}", inst);
        println!("PC: {:#X}", self.pc);

        // The highest nibble encodes the kind of instruction to be executed.
        let highest_nibble = inst >> 12;

        // The remaining nibbles or combination of remaining nibbles can encode certain information such as
        // registers, immediate numbers, or memory addresses. Extract them all out here and the proper one
        // will be used inside the instruction.
        let second_nibble = (inst >> 8) & 0xF;
        let third_nibble = (inst >> 4) & 0xF;
        let fourth_nibble = inst & 0xF;
        let lsb = inst & 0xFF;
        let remaining_nibbles = inst & 0xFFF;

        // println!("    second nibble: {second_nibble:#X}");
        // println!("    third nibble:  {third_nibble:#X}");
        // println!("    fourth nibble: {fourth_nibble:#X}");
        // println!("    lsb:           {lsb:#X}");
        // println!("    remaning nbls: {remaining_nibbles:#X}");

        match highest_nibble {
            0x0 => {
                match remaining_nibbles {
                    // Clear the screen.
                    0x0E0 => {
                        self.display.clear();
                        self.rerender = true;
                    }

                    // Return from subroutine.
                    0x0EE => {
                        self.pop_stack();
                    }

                    _ => {}
                }
            }
            0x1 => {
                self.pc = remaining_nibbles;
            }
            0x2 => {
                self.push_stack();
                self.pc = remaining_nibbles;
            }
            0x3 => self.skip_inst_if(self.gpvr[second_nibble as usize] == lsb as u8),
            0x4 => self.skip_inst_if(self.gpvr[second_nibble as usize] != lsb as u8),
            0x5 => self.skip_inst_if(
                self.gpvr[second_nibble as usize] == self.gpvr[third_nibble as usize],
            ),
            0x6 => {
                self.gpvr[second_nibble as usize] = lsb as u8;
            }
            0x7 => {
                self.gpvr[second_nibble as usize] =
                    self.gpvr[second_nibble as usize].saturating_add(lsb as u8)
            }
            0x8 => {
                let regx = second_nibble as usize;
                let regy = third_nibble as usize;

                match fourth_nibble {
                    0x0 => self.set_reg(regx, self.get_reg(regy)),
                    0x1 => {
                        let res = self.get_reg(regx) | self.get_reg(regy);
                        self.set_reg(regx, res);
                    }
                    0x2 => {
                        let res = self.get_reg(regx) & self.get_reg(regy);
                        self.set_reg(regx, res);
                    }
                    0x3 => {
                        let res = self.get_reg(regx) ^ self.get_reg(regy);
                        self.set_reg(regx, res);
                    }
                    0x4 => {
                        if self.get_reg(regx).checked_add(self.get_reg(regy)).is_none() {
                            *self.gpvr.last_mut().unwrap() = 1;
                        }
                        let res = self.get_reg(regx).saturating_add(self.get_reg(regy));
                        self.set_reg(regx, res)
                    }
                    0x5 => {
                        let regx_val = self.get_reg(regx);
                        let regy_val = self.get_reg(regy);

                        // Set the carry flag.
                        if regx_val > regy_val {
                            *self.gpvr.last_mut().unwrap() = 1;
                        } else {
                            *self.gpvr.last_mut().unwrap() = 0;
                        }

                        self.set_reg(regx, regx_val.saturating_sub(regy_val));
                    }
                    0x6 => self.inst_shiftr(regx, regy),
                    0x7 => {
                        let regx_val = self.get_reg(regx);
                        let regy_val = self.get_reg(regy);

                        // Set the carry flag.
                        if regy_val > regx_val {
                            *self.gpvr.last_mut().unwrap() = 1;
                        } else {
                            *self.gpvr.last_mut().unwrap() = 0;
                        }

                        self.set_reg(regx, regy_val - regx_val);
                    }
                    0xE => self.inst_shiftl(regx, regy),
                    _ => {}
                }
            }
            0x9 => self.skip_inst_if(
                self.gpvr[second_nibble as usize] != self.gpvr[third_nibble as usize],
            ),
            0xA => self.idxr = remaining_nibbles,
            0xB => {
                if cfg!(feature = "modern-jwo") {
                    self.pc = remaining_nibbles + self.get_reg(second_nibble as usize) as u16
                } else {
                    self.pc = remaining_nibbles + self.get_reg(0) as u16
                }
            }
            0xC => {
                let rand = rand::random::<u8>();
                self.set_reg(second_nibble as usize, rand & lsb as u8);
            }
            0xD => {
                let sprite_height = fourth_nibble as usize;

                *self.gpvr.last_mut().unwrap() = 0;

                for y in 0..sprite_height {
                    let sprite_byte = self.read_byte(self.idxr as usize + y);
                    let ycoord = ((self.get_reg(third_nibble as usize) as usize + y as usize)
                        % HEIGHT)
                        * SCALE;

                    for bit in (0..u8::BITS).rev() {
                        let xcoord = ((self.get_reg(second_nibble as usize) as usize
                            + bit as usize)
                            % WIDTH)
                            * SCALE;
                        let bit_val = (sprite_byte >> bit) & 0x1;

                        if bit_val == 1 {
                            if self.display.get_pixel(xcoord, ycoord) {
                                *self.gpvr.last_mut().unwrap() = 1;
                            }
                            self.display.toggle_pixel(xcoord, ycoord);
                        }
                    }
                }

                self.rerender = true;
            }
            0xE => match (third_nibble, fourth_nibble) {
                (0x9, 0xE) => self.skip_inst_if(
                    self.keyboard
                        .is_key_pressed(self.get_reg(second_nibble as usize) as usize),
                ),
                (0xA, 0x1) => self.skip_inst_if(
                    !self
                        .keyboard
                        .is_key_pressed(self.get_reg(second_nibble as usize) as usize),
                ),
                _ => {}
            },
            0xF => match (third_nibble, fourth_nibble) {
                (0x0, 0x7) => self.set_reg(second_nibble as usize, self.dtr),
                (0x0, 0xA) => loop {
                    let event = self.event_pump.wait_event();

                    if let Event::KeyDown {
                        scancode: Some(scancode),
                        ..
                    } = event
                    {
                        self.keyboard.press_key(scancode);
                        let scancode_hex = match scancode {
                            Scancode::Num1 => 0,
                            Scancode::Num2 => 1,
                            Scancode::Num3 => 2,
                            Scancode::Num4 => 3,
                            Scancode::Q => 4,
                            Scancode::W => 5,
                            Scancode::E => 6,
                            Scancode::R => 7,
                            Scancode::A => 8,
                            Scancode::S => 9,
                            Scancode::D => 10,
                            Scancode::F => 11,
                            Scancode::Z => 12,
                            Scancode::X => 13,
                            Scancode::C => 14,
                            Scancode::V => 15,
                            _ => panic!("unrecognized key"),
                        };
                        self.set_reg(second_nibble as usize, scancode_hex);
                        break;
                    }
                },
                (0x1, 0x5) => self.dtr = self.get_reg(second_nibble as usize),
                (0x1, 0x8) => self.str = self.get_reg(second_nibble as usize),
                (0x1, 0xE) => {
                    let regx_val = self.get_reg(second_nibble as usize);
                    let res = self.idxr + regx_val as u16;

                    if res >= MEM_SIZE as u16 {
                        *self.gpvr.last_mut().unwrap() = 1;
                    }
                }
                (0x2, 0x9) => self.idxr = self.get_reg(second_nibble as usize) as u16,
                (0x3, 0x3) => {
                    let num = self.get_reg(second_nibble as usize);

                    if let Some(log_base_ten) = num.checked_ilog10() {
                        let dig_count = log_base_ten + 1;

                        for pow in (0..dig_count).rev() {
                            let digit = ((num as u32 / 10_u32.pow(pow)) % 10) as u8;
                            self.write_byte(self.idxr as usize, digit);
                            self.idxr += 1;
                        }
                    } else {
                        self.write_byte(self.idxr as usize, num);
                    }
                }
                (0x5, 0x5) => {
                    if cfg!(feature = "modern-ls") {
                        let mut addr = self.idxr as usize;

                        for reg in 0..=second_nibble {
                            self.write_byte(addr, self.get_reg(reg as usize));
                            addr += 1;
                        }
                    } else {
                        for reg in 0..=second_nibble {
                            self.write_byte(self.idxr as usize, self.get_reg(reg as usize));
                            self.idxr += 1;
                        }
                    }
                }
                (0x6, 0x5) => {
                    if cfg!(feature = "modern-ls") {
                        let mut addr = self.idxr as usize;

                        for reg in 0..=second_nibble {
                            let val = self.read_byte(addr);
                            self.set_reg(reg as usize, val);
                            addr += 1;
                        }
                    } else {
                        for reg in 0..=second_nibble {
                            let val = self.read_byte(self.idxr as usize);
                            self.set_reg(reg as usize, val);
                            self.idxr += 1;
                        }
                    }
                }
                _ => {}
            },
            _ => panic!("CHIP-8 instructions only start with valid hex digits"),
        }
    }

    /// Execute the program.
    pub fn execute_program(&mut self, prog: &[u8], step: bool, no_display: bool) {
        // Load the program into memory.
        self.load_program(prog);

        if !no_display {
            // Render the initial, unmainpulated display.
            self.display.render();
        }

        loop {
            let event = self.event_pump.wait_event();

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break,
                Event::KeyDown {
                    scancode: Some(Scancode::N),
                    ..
                } if step => {
                    let inst = self.next_inst();
                    self.execute_instruction(inst);
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => self.keyboard.release_key(scancode),
                _ => {}
            }

            if !step {
                let inst = self.next_inst();
                self.execute_instruction(inst);
            }

            if self.rerender && !no_display {
                self.display.render();
            }
        }
    }

    /// Dump CPU state at the end of execution.
    pub fn dump_state(&self) {
        println!("\nPOST-EXECUTION CPU STATE");
        println!("------------------------");
        println!("PROGRAM COUNTER: {:#X}", self.pc);
        println!("INDEX REGISTER:  {:#X}", self.idxr);
        println!("DELAY TIMER:     {:#X}", self.dtr);
        println!("SOUND TIMER:     {:#X}", self.str);

        println!("GENERAL PURPOSE REGISTERS:");

        for (reg, val) in self.gpvr.iter().enumerate() {
            println!("    V{reg:X}:  {val:#X}");
        }
    }
}
