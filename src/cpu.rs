use arrayvec::ArrayVec;
use sdl2::render::WindowCanvas;

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

/// CPU state.
pub struct Cpu {
    /// Accessible memory (4096 bytes).
    mem: [u8; 4096],

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
}

impl Cpu {
    /// Create a new CPU initialized with default values.
    pub fn new() -> Self {
        Self {
            mem: std::array::from_fn(|_| 0),
            pc: 0,
            idxr: 0,
            gpvr: std::array::from_fn(|_| 0),
            stack: ArrayVec::new(),
            dtr: 0,
            str: 0,
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

    /// Implementation of the shift right instruction.
    fn inst_shiftr(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);

        if cfg!(feature = "shift-1990s") {
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

        if cfg!(feature = "shift-1990s") {
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
    pub fn next_inst(&mut self) -> u16 {
        let inst = self.read_word(self.pc as usize);
        self.pc += 2;
        inst
    }

    /// Load a program into memory and prepare for execution.
    pub fn load_program(&mut self, prog: &[u8]) {
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
    pub fn execute_instruction(&mut self, inst: u16, canvas: &mut WindowCanvas) {
        // println!("inst: {:#X}", inst);

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
                    0x0E0 => canvas.clear(),

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
            0x3 => {
                if self.gpvr[second_nibble as usize] == lsb as u8 {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.gpvr[second_nibble as usize] != lsb as u8 {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.gpvr[second_nibble as usize] == self.gpvr[third_nibble as usize] {
                    self.pc += 2;
                }
            }
            0x6 => {
                self.gpvr[second_nibble as usize] = lsb as u8;
            }
            0x7 => {
                self.gpvr[second_nibble as usize] += lsb as u8;
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
                        let res = self.get_reg(regx) + self.get_reg(regy);
                        // TODO: set carry.
                        self.set_reg(regx, res)
                    }
                    0x5 => {
                        let regx_val = self.get_reg(regx);
                        let regy_val = self.get_reg(regy);

                        // Set the carry flag.
                        if regx_val > regy_val {
                            *self.gpvr.last_mut().unwrap() = 1;
                        } else {
                            todo!();
                        }

                        self.set_reg(regx, regx_val - regy_val);
                    }
                    0x6 => self.inst_shiftr(regx, regy),
                    0x7 => {
                        let regx_val = self.get_reg(regx);
                        let regy_val = self.get_reg(regy);

                        // Set the carry flag.
                        if regy_val > regx_val {
                            *self.gpvr.last_mut().unwrap() = 1;
                        } else {
                            todo!();
                        }

                        self.set_reg(regx, regy_val - regx_val);
                    }
                    0xE => self.inst_shiftl(regx, regy),
                    _ => {}
                }
            }
            0x9 => {
                if self.gpvr[second_nibble as usize] != self.gpvr[third_nibble as usize] {
                    self.pc += 2;
                }
            }
            0xA => self.idxr = remaining_nibbles,
            0xB => self.pc = remaining_nibbles + self.get_reg(0) as u16,
            0xC => {
                let rand = rand::random::<u8>();
                self.set_reg(second_nibble as usize, rand & lsb as u8);
            }
            0xD => todo!(),
            0xE => todo!(),
            0xF => match (third_nibble, fourth_nibble) {
                (0x0, 0x7) => self.set_reg(second_nibble as usize, self.dtr),
                (0x0, 0xA) => todo!(),
                (0x1, 0x5) => self.dtr = self.get_reg(second_nibble as usize),
                (0x1, 0x8) => self.str = self.get_reg(second_nibble as usize),
                (0x1, 0xE) => todo!(),
                (0x2, 0x9) => todo!(),
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
                    for reg in 0..=second_nibble {
                        self.write_byte(self.idxr as usize, self.get_reg(reg as usize));
                        self.idxr += 1;
                    }
                }
                (0x6, 0x5) => {
                    for reg in 0..=second_nibble {
                        let val = self.read_byte(self.idxr as usize);
                        self.set_reg(reg as usize, val);
                        self.idxr += 1;
                    }
                }
                _ => {}
            },
            _ => panic!("CHIP-8 instructions only start with valid hex digits"),
        }
    }

    /// Dump CPU state at the end of execution.
    pub fn dump_state(&self) {
        println!("PROGRAM COUNTER: {:#X}", self.pc);
        println!("INDEX REGISTER:  {:#X}", self.idxr);

        println!("GENERAL PURPOSE REGISTERS:");

        for (reg, val) in self.gpvr.iter().enumerate() {
            println!("    V{reg:X}:  {val:#X}");
        }

        println!("DELAY TIMER:    {:#X}", self.dtr);
        println!("SOUND TIMER:    {:#X}", self.str);
    }
}
