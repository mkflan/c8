#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod cpu;
mod display;
mod keyboard;

use clap::Parser;
use cpu::Cpu;
use display::{Display, HEIGHT, SCALE, WIDTH};
use std::{error::Error, fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
#[command(author, about, version, propagate_version = true)]
struct Cli {
    /// Path to the CHIP-8 program.
    prog_path: PathBuf,

    /// Step through instructions individually.
    #[arg(short, long)]
    step: bool,

    /// Prevent the display from being shown.
    #[arg(long)]
    no_display: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Setup needed sdl2 facilities.
    let sdl_context = sdl2::init()?;
    let window = sdl_context
        .video()?
        .window(
            "C8: CHIP-8 Emulator",
            (WIDTH * SCALE) as u32,
            (HEIGHT * SCALE) as u32,
        )
        .position_centered()
        .resizable()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let event_pump = sdl_context.event_pump()?;

    let display = Display::new(canvas);
    let mut cpu = Cpu::new(display, event_pump);

    // Read the program instructions into a buffer.
    let mut rom = File::open(args.prog_path)?;
    let mut prog = Vec::new();
    rom.read_to_end(&mut prog)?;

    // Execute the program.
    cpu.execute_program(&prog, args.step, args.no_display);

    // Dump post-execution state.
    cpu.dump_state();

    Ok(())
}
