pub mod lexer;
pub mod token;

use std::{fmt, iter::Peekable};
use token::{Mnemonic, Register, Token, TokenKind};

/// An instruction operand.
#[derive(Debug, Clone, Copy)]
pub enum Operand {
    /// A register.
    Register(Register),

    /// An up to 12-bit immediate number.
    Immediate(u16),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operand::*;

        write!(
            f,
            "{}",
            match self {
                Register(r) => r.to_string(),
                Immediate(imm) => format!("{imm:#04X}"),
            }
        )
    }
}

/// An instruction, with its mnemonic and operands.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// The instruction mnemonic.
    pub mnemonic: Mnemonic,

    /// The instruction operands.
    pub operands: Vec<Operand>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{mnemonic} {operands}",
            mnemonic = self.mnemonic,
            operands = self
                .operands
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    most_recently_consumed: Option<Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            most_recently_consumed: None,
        }
    }

    /// Peek the next token.
    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Get the next token.
    fn next_token(&mut self) -> Option<Token> {
        let next = self.tokens.next();
        self.most_recently_consumed = next;
        next
    }

    /// Returns whether the next token meets the given predicate, consuming it if it is.
    fn next_is(&mut self, pred: impl FnOnce(&Token) -> bool) -> bool {
        self.peek_token()
            .is_some_and(pred)
            .then(|| self.next_token())
            .flatten()
            .is_some()
    }

    pub fn parse_instruction(&mut self) -> Option<Instruction> {
        let mnemonic = self.next_token()?;

        if let TokenKind::Mnemonic(m) = mnemonic.kind {
            let mut operands = Vec::new();

            while self.next_is(|t| !matches!(t.kind, TokenKind::Mnemonic(_))) {
                if matches!(
                    self.most_recently_consumed.unwrap().kind,
                    TokenKind::Comma | TokenKind::EoF
                ) {
                    continue;
                }

                let kind = self.most_recently_consumed.unwrap().kind;
                if let TokenKind::Immediate(imm) = kind {
                    operands.push(Operand::Immediate(imm));
                } else if let TokenKind::Register(reg) = kind {
                    operands.push(Operand::Register(reg));
                }
            }

            return Some(Instruction {
                mnemonic: m,
                operands,
            });
        }

        None
    }
}
