#![allow(unused, dead_code)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

use std::{error::Error, fs, path::PathBuf};

pub fn assemble(asm_path: PathBuf, outfile: PathBuf) -> Result<(), Box<dyn Error>> {
    let asm_code = fs::read_to_string(asm_path)?;

    Ok(())
}
