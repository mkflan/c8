#![allow(unused, dead_code)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

use std::{error::Error, fs, path::PathBuf};

// MNEMONIC   INST            INST DESC
// cls        00E0            clear screen
// jmp        1NNN            jump
// csrt       2NNN            call subroutine
// rsrt       00EE            return from subroutine
// seq        3XNN/5XY0       skip next instruction if operands are equal
// sneq       4XNN/9XY0       skip next instruction if operands are not equal
// set                        set the value of a register
//            6XNN            - set VX to immediate 8 bit number
//            8XY0            - set VX to the value of VY
//            FX07            - set VX to the value of the delay register

// setv       6XNN/8XY0/FX07  set variable register (supported operands: immediate 8-bit number, variable register, delay register)
// add        7XNN            add immediate value to value in VX
// bwor       8XY1            bitwise OR VX's value with VY's value, storing the result in VX
// bwand      8XY2            bitwise AND VX's value with VY's value, storing the result in VX
// bwxor      8XY3            bitwise XOR VX's value with VY's value, storing the result in VX
// add        8XY4            add value of VY to VX (affects VF)
// sub        8XY5            subtract VY from VX, storing result in VX (affects VF)
// subb       8XY7            subtract VX from VY, storing result in VX (affects VF)
// sftr       8XY6            shift the value in VX right by one (affects VF)
// sftl       8XYE            shift the value in VX left by one (affects VF)
// seti       ANNN            set index register with immediate 12-bit memory address
// jmpwo      BNNN            jump with offset
// rand       CXNN            generate a random number, storing it bitwise ANDed with immediate 8-bit number in VX
// draw       DXYN            draw `N` pixels tall sprite at X coord specified by VX's value and Y coord specified by VY's value.
// skp        EX9E            skip next instruction if key corresponding to VX's is pressed
// sknp       EXA1            skip next instruction if key corresponding to VX's is not pressed
// setd       FX15            set delay timer to value in VX
// sets       FX18            set sound timer to value in VX
// gk         FX0A            block until a key is pressed, storing the hex value of the pressed key in VX

// FX29 TODO
// FX1E TODO

pub fn assemble(asm_path: PathBuf, outfile: PathBuf) -> Result<(), Box<dyn Error>> {
    let asm_code = fs::read_to_string(asm_path)?;

    Ok(())
}
