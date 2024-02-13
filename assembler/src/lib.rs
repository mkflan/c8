#![allow(unused, dead_code)]
#![feature(lazy_cell)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod parser;

use parser::{
    lexer::Lexer,
    token::{Token, TokenKind},
};
use std::{error::Error, fs, path::PathBuf};

pub fn assemble(asm_path: PathBuf, outfile: PathBuf) -> Result<(), Box<dyn Error>> {
    let asm_code = fs::read_to_string(asm_path)?;

    let mut lexer = Lexer::new(asm_code.chars());

    // TODO: use std::iter::from_fn
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lexer.lex_token() {
        tokens.push(token);

        if token.kind == TokenKind::EoF {
            break;
        }
    }

    dbg!(tokens);

    Ok(())
}
