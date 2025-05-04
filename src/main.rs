#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]
#![allow(unused)]

mod cpu;
mod keyboard;

use clap::Parser;
use cpu::Cpu;
use sdl2::{event::Event, keyboard::Scancode, pixels::PixelFormatEnum, render::WindowCanvas};
use std::{error::Error, fs::File, io::Read, path::PathBuf};

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_SCALE: usize = 10;
pub const BG_COLOR: u8 = u8::MIN; // black background
pub const FG_COLOR: u8 = u8::MAX; // white foreground

#[derive(Parser)]
#[command(author, about, version, propagate_version = true)]
struct Cli {
    /// Path to the CHIP-8 program.
    prog_path: PathBuf,

    /// Step through instructions individually.
    #[arg(short, long)]
    step: bool,
}

fn render_display(
    canvas: &mut WindowCanvas,
    framebuffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    init: bool,
) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(
            PixelFormatEnum::RGB332,
            DISPLAY_WIDTH as u32,
            DISPLAY_HEIGHT as u32,
        )
        .expect("unable to create texture");

    let pixel_data = framebuffer.map(|p| {
        if init || (!init && !p) {
            BG_COLOR
        } else {
            FG_COLOR
        }
    });

    texture
        .update(None, &pixel_data, DISPLAY_WIDTH * size_of::<u8>())
        .expect("unable to update texture");

    canvas.clear();
    canvas
        .copy(&texture, None, None)
        .expect("unable to copy texture");
    canvas.present();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Setup needed sdl2 facilities.
    let sdl_context = sdl2::init()?;
    let window = sdl_context
        .video()?
        .window(
            "C8: CHIP-8 Emulator",
            DISPLAY_WIDTH as u32 * DISPLAY_SCALE as u32,
            DISPLAY_HEIGHT as u32 * DISPLAY_SCALE as u32,
        )
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    // Read the program instructions into a buffer.
    let mut rom = File::open(args.prog_path)?;
    let mut prog = Vec::new();
    rom.read_to_end(&mut prog)?;

    println!("{rom:#?}");

    let mut cpu = Cpu::new();

    // Load the program into memory.
    cpu.load_program(&prog);

    // Initialize the display with a black background.
    render_display(&mut canvas, cpu.display, true);

    loop {
        let inst = cpu.next_inst();
        let event = event_pump.wait_event();

        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } => break,
            Event::KeyDown {
                scancode: Some(Scancode::N),
                ..
            } if args.step => {
                cpu.execute_instruction(inst);
            }
            // Event::KeyDown {
            //     scancode: Some(scancode),
            //     ..
            // } => self.keyboard.press_key(scancode),
            // Event::KeyUp {
            //     scancode: Some(scancode),
            //     ..
            // } => self.keyboard.release_key(scancode),
            _ => {}
        }

        if !args.step {
            cpu.execute_instruction(inst);
        }

        if cpu.rerender {
            render_display(&mut canvas, cpu.display, false);
        }
    }

    // Dump post-execution state.
    cpu.dump_state();

    Ok(())
}
