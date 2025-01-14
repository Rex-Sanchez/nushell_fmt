use crate::tokens::{Token, TokonizerTools};

pub struct Tokonizer {
    pub index: usize,
    pub tokens: Vec<Token>,
    pub stack: Vec<Token>,
    pub temp: String,
}

impl Tokonizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            stack: Vec::new(),
            temp: String::new(),
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
                return Some(token.clone());
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
    pub fn one_of_is_eq(&self, token: &[Token]) -> bool {
        if let Some(current) = self.get() {
            for t in token {
                if current == *t {
                    return true;
                }
            }
        }
        false
    }
    pub fn to_stack(&mut self, token: Token) {
        self.stack.push(token.clone())
    }
    pub fn take_upto(&mut self, token: &[Token], included: bool) -> String {
        let mut block = String::new();

        let mut depth = 0;

        let mut first = true;

        loop {
            if let Some(t) = self.get() {
                match t {
                    Token::BraceOpen => {
                        depth += 1;
                    }
                    Token::BraceClose => {
                        depth -= 1;
                    }
                    _ => {}
                }

                if !first && self.one_of_is_eq(token) {
                    if depth == 0 {
                        if included {
                            block.push_str(&t.as_string());
                        } else {
                            self.prev();
                        }
                        break;
                    }
                }

                block.push_str(&t.as_string());
                self.next();
                first = false;
                continue;
            }
            break;
        }

        block
    }
    pub fn take_upto_either_included_if(
        &mut self,
        token: &[Token],
        if_included: Token,
    ) -> String {
        let mut block = String::new();
        while !self.one_of_is_eq(token) {
            if let Some(t) = self.get() {
                block.push_str(&t.as_string());
                self.next()
            }
        }
        if let Some(t) = self.get() {
            if if_included == t {
                block.push_str(&t.as_string());
            }
        }
        block
    }

    pub fn to_temp(&mut self, s: String) {
        self.temp.push_str(&s);
    }
    pub fn temp_to_word(&mut self) {
        if !self.temp.is_empty() {
            self.to_stack(Token::Word(self.temp.clone()));
            self.temp.clear();
        }
    }
}
