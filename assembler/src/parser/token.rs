use std::ops::Range;

/// An instruction mnemonic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    Cls,
    Jmp,
    Csrt,
    Rsrt,
    Seq,
    Sneq,
    Set,
    Add,
    BwOr,
    BwAnd,
    BwXor,
    Sub,
    Subb,
    Sftr,
    Sftl,
    Jmpwo,
    Rand,
    Draw,
    Skp,
    Sknp,
    Gk,
    Bcd,
    Stm,
    Ldm,
}

/// A special register name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialReg {
    Index,
    Delay,
    Sound,
}

/// Valid tokens that the lexer will recognize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// An instruction mnemonic.
    Mnemonic(Mnemonic),

    /// A special register name.
    SpecialReg(SpecialReg),

    /// An up to 12-bit immediate number.
    Immediate(u16),

    /// A variable register.
    VarReg(u8),

    /// A comma (,).
    Comma,

    /// End of File
    EoF,
}

/// Metadata given to a token to indicate its position within the source code.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

/// A token, with a span indicating its position in the source code.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// Create a new token with the given kind and span.
    pub fn new(kind: TokenKind, span: impl Into<Span>) -> Self {
        Self {
            kind,
            span: span.into(),
        }
    }
}
