use miette::SourceSpan;
use std::{fmt, ops::Range};

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

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Mnemonic::*;

        write!(
            f,
            "{}",
            match self {
                Cls => "cls",
                Jmp => "jmp",
                Csrt => "csrt",
                Rsrt => "rsrt",
                Seq => "seq",
                Sneq => "sneq",
                Set => "set",
                Add => "add",
                BwOr => "bwor",
                BwAnd => "bwand",
                BwXor => "bwxor",
                Sub => "sub",
                Subb => "subb",
                Sftr => "sftr",
                Sftl => "stfl",
                Jmpwo => "jmpwo",
                Rand => "rand",
                Draw => "draw",
                Skp => "skp",
                Sknp => "sknp",
                Gk => "gk",
                Bcd => "bcd",
                Stm => "stm",
                Ldm => "ldm",
            }
        )
    }
}

/// A register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// A variable register.
    VarReg(u8),

    /// The INDEX register.
    Index,

    /// The DELAY register.
    Delay,

    /// The SOUND register.
    Sound,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Register::*;

        write!(
            f,
            "{}",
            match self {
                VarReg(r) => format!("V{r:X}"),
                Index => "index".to_string(),
                Delay => "delay".to_string(),
                Sound => "sound".to_string(),
            }
        )
    }
}

/// Valid tokens that the lexer will recognize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// An instruction mnemonic.
    Mnemonic(Mnemonic),

    /// A register.
    Register(Register),

    /// An up to 12-bit immediate number.
    Immediate(u16),

    /// A comma (,).
    Comma,

    /// A character the lexer does not recognize.
    Unrecognized,

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

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        Self::from(span.start..span.end)
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
