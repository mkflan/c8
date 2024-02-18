#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod cpu;
mod display;
mod keyboard;

use clap::{Parser, Subcommand};
use cpu::Cpu;
use display::{Display, HEIGHT, SCALE, WIDTH};
use std::{error::Error, fs::File, io::Read, path::PathBuf};

#[derive(Subcommand)]
enum Commands {
    /// Assemble a CHIP-8 Assembly file.
    Assemble {
        /// Path to the CHIP-8 Assembly file.
        asm_path: PathBuf,

        /// Path to output assembled code.
        #[arg(short, long, default_value = "prog.c8out")]
        outfile: PathBuf,
    },

    /// Emulate a CHIP-8 program.
    Emulate {
        /// Path to the CHIP-8 program.
        prog_path: PathBuf,

        /// Step through instructions individually.
        #[arg(short, long)]
        step: bool,

        /// Prevent the display from being shown.
        #[arg(long)]
        no_display: bool,
    },
}

#[derive(Parser)]
#[command(author, about, version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn emulate(prog_path: PathBuf, step: bool, no_display: bool) -> Result<(), Box<dyn Error>> {
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
    let mut rom = File::open(prog_path)?;
    let mut prog = Vec::new();
    rom.read_to_end(&mut prog)?;

    // Execute the program.
    cpu.execute_program(&prog, step, no_display);

    // Dump post-execution state.
    cpu.dump_state();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Assemble { asm_path, outfile } => Ok(assembler::assemble(asm_path, outfile)?),
        Commands::Emulate {
            prog_path,
            step,
            no_display,
        } => emulate(prog_path, step, no_display),
    }
}
