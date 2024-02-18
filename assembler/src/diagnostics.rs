use crate::parser::token::Span;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum AssemblerDiagnostic {
    #[error("Encountered unrecognized character in source code.")]
    #[diagnostic(code(c8::assembler::parser::unrecognized_char))]
    UnrecognizedChar(#[label("here")] Span),

    #[error("Encountered unrecognized identifier in source code.")]
    #[diagnostic(code(c8::assembler::parser::unrecognized_identifier))]
    UnrecognizedIdent(#[label("here")] Span),

    #[error("Invalid immediate hex number.")]
    #[diagnostic(code(c8::assembler::parser::invalid_immediate_hexnumber))]
    InvalidImmediate(#[label("here")] Span),
}

#[derive(Debug, Error, Diagnostic, Default)]
#[error("")]
pub struct DiagnosticSink {
    #[related]
    pub diags: Vec<AssemblerDiagnostic>,
}
