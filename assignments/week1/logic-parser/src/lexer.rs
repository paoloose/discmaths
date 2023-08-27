use std::ops::Not;
use crate::errors::LexerError;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Proposition(String),
    Literal(bool),
    Not,
    And,
    Or,
    Implies,
    OpenParen,
    CloseParen
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    len: usize
}

pub struct Lexer<'a> {
    expr: &'a str,
    pos: usize
}

impl<'a> Lexer<'a> {
    pub fn new(expr: &'a str) -> Lexer {
        Lexer { expr: expr.clone(), pos: 0 }
    }

    pub fn parse(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        return Ok(tokens);
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespaces();
        let start = self.pos;

        let c = match self.next_char() {
            Some(c) => c,
            None => return Ok(None)
        };

        let token = match c {
            '(' => Token { kind: TokenKind::OpenParen, start, len: 1 },
            ')' => Token { kind: TokenKind::CloseParen, start, len: 1 },
            '!' => Token { kind: TokenKind::Not, start, len: 1 },
            '&' => Token { kind: TokenKind::And, start, len: 1 },
            '|' => Token { kind: TokenKind::Or, start, len: 1 },
            '=' => {
                match self.next_char() {
                    Some('>') => Token { kind: TokenKind::Implies, start, len: 1 },
                    _ => return Err(LexerError::SyntaxError(format!("Unexpected character (expected '=>'), got '{}'", c)))
                }
            },
            c @ '_' | c if c.is_alphabetic() => self.tokenize_proposition(start),
            _ => return Err(LexerError::UnkownToken(format!("Unkown Token: '{}'", c)))
        };

        Ok(Some(token))
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.expr[self.pos..].chars().next();
        match c {
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            },
            None => None,
        }
    }

    fn tokenize_proposition(&mut self, start: usize) -> Token {
        // We add one because we already consumed the first character
        let token_len = self.take_while(|c| c.is_alphanumeric()) + 1;
        println!("len={token_len}, start={start}");
        let p = &self.expr[start..start + token_len];
        if p == "false" || p == "true" {
            Token { kind: TokenKind::Literal(if p == "true" { true } else { false }), start, len: token_len }
        }
        else {
            Token { kind: TokenKind::Proposition(p.into()), start, len: token_len }
        }
    }

    fn skip_whitespaces(&mut self) -> usize {
        self.take_while(|c| c == '\t' || c == ' ' || c == '\r')
    }

    fn take_while<F>(&mut self, pred: F) -> usize
    where F: Fn(char) -> bool {
        let from = self.pos;

        for c in self.expr[self.pos..].chars() {
            if pred(c).not() {
                break;
            }
            self.pos += c.len_utf8();
        }

        self.pos - from
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_whitespaces_should_return_zero() {
        let expr = "testing => this";
        let n = Lexer::skip_whitespaces(&mut Lexer::new(expr));
        assert_eq!(n, 0);
    }

    #[test]
    fn literal_booleans_work() {
        let lexer = Lexer::new("false & true");
        let tokens = lexer.parse().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Literal(false));
        assert_eq!(tokens[1].kind, TokenKind::And);
        assert_eq!(tokens[2].kind, TokenKind::Literal(true));
    }

    #[test]
    fn take_while_returns_zero_if_no_matches() {
        let mut lexer = Lexer::new("testing");
        let n = lexer.take_while(|_| false);
        assert_eq!(n, 0);
    }

    #[test]
    fn take_while_returns_correct_amount() {
        let mut lexer = Lexer::new("kittens");
        let n = lexer.take_while(|_| true);
        assert_eq!(n, 7);
    }

    #[test]
    fn skip_whitespaces_works_properly() {
        let lexer = Lexer::new("\t\r puppies");
        let tokens = lexer.parse().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Proposition("puppies".into()));
    }

    #[test]
    #[should_panic]
    fn propositions_cant_start_with_numbers() {
        let lexer = Lexer::new("pqrs");
        if !lexer.parse().is_ok() { return; }

        let lexer = Lexer::new("69p");
        let _ = lexer.parse().unwrap();
    }
}
