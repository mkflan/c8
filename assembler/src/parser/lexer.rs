use crate::diagnostics::AssemblerDiagnostic;

use super::token::{
    Register::*,
    Span, Token,
    TokenKind::{self, *},
};
use std::{collections::HashMap, iter::Peekable, sync::LazyLock};

static RESERVED_IDENTIFIERS: LazyLock<HashMap<&str, TokenKind>> = LazyLock::new(|| {
    use super::token::Mnemonic::*;

    HashMap::from([
        ("cls", Mnemonic(Cls)),
        ("jmp", Mnemonic(Jmp)),
        ("csrt", Mnemonic(Csrt)),
        ("rsrt", Mnemonic(Rsrt)),
        ("seq", Mnemonic(Seq)),
        ("sneq", Mnemonic(Sneq)),
        ("set", Mnemonic(Set)),
        ("add", Mnemonic(Add)),
        ("bwor", Mnemonic(BwOr)),
        ("bwand", Mnemonic(BwAnd)),
        ("bwxor", Mnemonic(BwXor)),
        ("sub", Mnemonic(Sub)),
        ("subb", Mnemonic(Subb)),
        ("sftr", Mnemonic(Sftr)),
        ("sftl", Mnemonic(Sftl)),
        ("jmpwo", Mnemonic(Jmpwo)),
        ("rand", Mnemonic(Rand)),
        ("draw", Mnemonic(Draw)),
        ("skp", Mnemonic(Skp)),
        ("sknp", Mnemonic(Sknp)),
        ("gk", Mnemonic(Gk)),
        ("bcd", Mnemonic(Bcd)),
        ("stm", Mnemonic(Stm)),
        ("ldm", Mnemonic(Ldm)),
        ("index", Register(Index)),
        ("delay", Register(Delay)),
        ("sound", Register(Sound)),
    ])
});

pub struct Lexer<I: Iterator<Item = char>> {
    /// An iterator over the source code.
    source: Peekable<I>,

    /// Where the lexer currently is in the source code.
    cursor: (usize, usize),

    /// The most recently consumed character.
    most_recently_consumed: Option<char>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(source: I) -> Self {
        Self {
            source: source.peekable(),
            cursor: (1, 0),
            most_recently_consumed: None,
        }
    }

    /// Create a new token.
    fn create_token(&self, kind: TokenKind, token_len: usize) -> Token {
        Token::new(kind, self.cursor.1 - token_len..self.cursor.1)
    }

    /// View the next character without advancing the source iterator.
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    /// Advance to the next character in the source iterator.
    fn advance(&mut self) -> Option<char> {
        self.cursor.1 += 1;
        let next = self.source.next();
        self.most_recently_consumed = next;
        next
    }

    /// Returns whether the next character meets the given predicate, consuming it if it is.
    fn next_is(&mut self, pred: impl FnOnce(&char) -> bool) -> bool {
        self.peek()
            .is_some_and(pred)
            .then(|| self.advance())
            .flatten()
            .is_some()
    }

    /// Advance while a predicate is true.
    fn advance_while(&mut self, pred: impl Fn(&char) -> bool) {
        while self.next_is(&pred) {}
    }

    /// Lex a reserved identifier, whether it be an instruction mnemonic or special register name.
    fn lex_reserved_ident(&mut self, first: char) -> (Token, Option<AssemblerDiagnostic>) {
        let mut err = None;
        let mut ident = String::from(first);

        while self.next_is(|c| c.is_alphabetic()) {
            ident.push(self.most_recently_consumed.unwrap());
        }

        let Some(&tok_kind) = (*RESERVED_IDENTIFIERS).get(ident.as_str()) else {
            let tok = self.create_token(Unrecognized, ident.len());
            err = Some(AssemblerDiagnostic::UnrecognizedIdent(tok.span));
            return (tok, err);
        };

        (self.create_token(tok_kind, ident.len()), err)
    }

    /// Lex an immediate hexadecimal numerical value.
    fn lex_immediate_hexnumber(&mut self) -> (Token, Option<AssemblerDiagnostic>) {
        let mut err = None;
        let num = std::iter::from_fn(|| {
            self.next_is(char::is_ascii_hexdigit)
                .then_some(self.most_recently_consumed)
                .flatten()
        })
        .collect::<String>();
        let imm_len = num.len() + 1; // Account for leading #.

        if let Ok(num) = u16::from_str_radix(&num, 16) {
            (self.create_token(Immediate(num), imm_len), None)
        } else {
            let span = Span::from(self.cursor.1 - imm_len + 1..imm_len);
            err = Some(AssemblerDiagnostic::InvalidImmediate(span));
            (self.create_token(Unrecognized, imm_len), err)
        }

        // let num = u16::from_str_radix(&num, 16)
        //     .ok().
        //     .expect("unable to parser immediate number to hex");
        // (self.create_token(Immediate(num), imm_len), None)
    }

    // TODO:
    // error handling:
    // - error if an immediate number is too big for the given instruction its passed as an argument to
    // - error on missing argument in instruction
    // - error on invalid instruction mnemonic
    // - error on invalid register name
    pub fn lex_token(&mut self) -> (Token, Option<AssemblerDiagnostic>) {
        let Some(ch) = self.advance() else {
            return (self.create_token(EoF, 0), None);
        };

        match ch {
            ',' => (self.create_token(Comma, 1), None),

            // Comments
            ';' => {
                self.advance_while(|&c| c != '\n');
                self.lex_token()
            }

            '#' => self.lex_immediate_hexnumber(),

            ch if ch.is_whitespace() => self.lex_token(),

            ch if ch.is_alphabetic() => {
                if (ch == 'V' || ch == 'v') && self.next_is(char::is_ascii_hexdigit) {
                    let reg = self
                        .most_recently_consumed
                        .map(|c| c.to_digit(16).unwrap() as u8)
                        .expect("unable to parse to hex");
                    return (self.create_token(Register(VarReg(reg)), 2), None);
                } else {
                    self.lex_reserved_ident(ch)
                }
            }

            _ => {
                let tok = self.create_token(Unrecognized, 1);
                (tok, Some(AssemblerDiagnostic::UnrecognizedChar(tok.span)))
            }
        }
    }
}
