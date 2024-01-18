#![allow(dead_code)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod cpu;
mod display;

use cpu::Cpu;
use display::*;
use std::{env, error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    // let step = args.contains(&String::from("--step"));
    let rom = args.get(0).expect("expected CHIP-8 ROM file");

    // Setup the sdl2 graphics.
    let sdl_context = sdl2::init()?;
    let window = sdl_context
        .video()?
        .window("C8: CHIP-8 Emulator", WIDTH * SCALE, HEIGHT * SCALE)
        .position_centered()
        .resizable()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let display = Display::new(canvas);
    let event_pump = sdl_context.event_pump()?;

    let mut cpu = Cpu::new(display, event_pump);

    // Read the program instructions into a buffer.
    let mut rom = File::open(rom)?;
    let mut prog = Vec::new();
    rom.read_to_end(&mut prog)?;

    // Execute the program.
    cpu.execute_program(&prog);

    // Dump post-execution state.
    cpu.dump_state();

    Ok(())
}
