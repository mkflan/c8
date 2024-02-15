pub mod lexer;
pub mod token;

use std::iter::Peekable;
use token::{Mnemonic, Token, TokenKind};

/// An instruction operand.
#[derive(Debug)]
enum Operand {
    /// A register.
    Register(TokenKind),

    /// An up to 12-bit immediate number.
    Immediate(u16),
}

/// An instruction, with its mnemonic and operands.
#[derive(Debug)]
pub struct Instruction {
    /// The instruction mnemonic.
    mnemonic: Mnemonic,

    /// The instruction operands.
    operands: Vec<Operand>,
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
                } else {
                    operands.push(Operand::Register(kind));
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
