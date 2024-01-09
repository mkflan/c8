#![allow(dead_code)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod cpu;

use cpu::Cpu;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{env, error::Error, fs::File, io::Read, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let rom = args.get(0).expect("expected CHIP-8 ROM file");

    let mut cpu = Cpu::new();

    // Read the program instructions into a buffer.
    let mut rom = File::open(Path::new("roms").join(rom))?;
    let mut prog = Vec::new();
    rom.read_to_end(&mut prog)?;

    // Load the program into memory.
    cpu.load_program(&prog);

    // Setup the sdl2 graphics.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("C8: CHIP-8 Emulator", 500, 250)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.clear();
    canvas.set_draw_color(Color::WHITE);
    canvas.present();

    'exec: loop {
        // canvas.draw_rect(sdl2::rect::Rect::new(50, 50, 25, 25))?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'exec,
                _ => {}
            }
        }

        let inst = cpu.next_inst();
        cpu.execute_instruction(inst, &mut canvas);

        canvas.present();
    }

    cpu.dump_state();

    Ok(())
}
