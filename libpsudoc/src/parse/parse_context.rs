use crate::coretypes::{Token, TokenCategory};

pub struct ParseContext {
    tokens: Vec<Token>,
    pub current: usize,
}

impl ParseContext {
    pub fn new(tokens: Vec<Token>) -> ParseContext {
        ParseContext { tokens, current: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.tokens.len() > self.current
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.has_next() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    pub fn last_read_token(&self) -> &Token {
        &self.tokens[std::cmp::max(1, std::cmp::min(self.tokens.len(), self.current)) - 1]
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.has_next() {
            let result = &self.tokens[self.current];
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }

    pub fn skip_whitespaces(&mut self) -> bool {
        let mut skipped = false;
        while let Some(token) = self.peek() {
            if token.category != TokenCategory::Whitespace {
                break;
            }
            self.next();
            skipped = true;
        }
        skipped
    }

    pub fn create_sandbox(&self) -> ParseContext {
        ParseContext {
            tokens: self
                .tokens
                .clone()
                .into_iter()
                .skip(self.current)
                .collect::<Vec<_>>(),
            current: 0,
        }
    }
}