
use crate::tokens::{Token, TokonizerTools};

pub struct Tokonizer {
   pub index: usize,
   pub tokens: Vec<Token>,
   pub stack: Vec<Token>,
}

impl Tokonizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            stack: Vec::new(),
        }
    }
    pub fn next(&mut self) {
        self.index += 1;
    }
    pub fn prev(&mut self) {
        self.index -= 1;
    }
    pub fn get(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }
    pub fn peak_next(&self) -> Option<Token> {
        let index = self.index.overflowing_add(1).to_option();
        self.tokens.get(index?).cloned()
    }
    pub fn peak_prev(&self) -> Option<Token> {
        let index = self.index.overflowing_sub(1).to_option();
        self.tokens.get(index?).cloned()
    }
    pub fn next_eq(&self, token: Token) -> bool {
        if self.peak_next() == Some(token) {
            return true;
        }
        false
    }
    pub fn peak_next_non_whitespace(&self, token: Token) -> Option<Token> {
        let mut index = self.index + 1;
        while let Some(token) = self.tokens.get(index) {
            if token != &Token::WhiteSpace {
                return Some(token.clone())
            }
            index += 1;
        }
        None
    }
    pub fn prev_eq(&self, token: Token) -> bool {
        if self.peak_prev() == Some(token) {
            return true;
        }
        false
    }
    pub fn is_eq(&self, token: &Token) -> bool {
        if self.get() == Some(token.clone()) {
            return true;
        }
        false
    }
    pub fn to_stack(&mut self, token: Token) {
        self.stack.push(token.clone())
    }
    pub fn take_upto(&mut self, token: Token) -> String {
        let mut block = String::new();
        while !self.is_eq(&token) {
            if let Some(token) = self.get() {
                block.push_str(&token.as_string());
                self.next()
            }
        }
        block
    }
}
