#![allow(dead_code, unused)]
#![feature(lazy_cell, let_chains)]
#![warn(rust_2018_idioms, clippy::pedantic, clippy::nursery)]

mod diagnostics;
mod encode;
// mod layout;
mod parser;

use diagnostics::DiagnosticSink;
use itertools::Itertools;
use miette::{IntoDiagnostic, NamedSource, Report};
use parser::{lexer::Lexer, token::TokenKind, Parser};
use std::{fs, path::PathBuf};

pub fn assemble(asm_path: PathBuf, outfile: PathBuf) -> miette::Result<()> {
    let asm_code = fs::read_to_string(&asm_path).unwrap();

    let mut lexer = Lexer::new(asm_code.chars());
    let mut errors = DiagnosticSink::default();
    let mut tokens = vec![];

    loop {
        let (token, diag) = lexer.lex_token();

        tokens.push(token);

        if diag.is_some() {
            errors.diags.push(diag.unwrap());
        }

        if token.kind == TokenKind::EoF {
            break;
        }
    }

    if !errors.diags.is_empty() {
        let report = Report::from(errors)
            .with_source_code(NamedSource::new(asm_path.display().to_string(), asm_code));
        return Err(report);
    }

    let mut parser = Parser::new(tokens.into_iter());
    let prog = std::iter::from_fn(|| parser.parse_instruction()).collect::<Vec<_>>();

    let prog_bytes = encode::encode_instructions(prog);

    for (&hi, &lo) in prog_bytes.iter().tuples() {
        let inst = ((hi as u16) << 8) | (lo as u16);
        println!("{inst:#04X}");
    }

    fs::write(outfile, prog_bytes).into_diagnostic()?;

    Ok(())
}
