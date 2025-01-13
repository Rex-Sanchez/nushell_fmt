#![allow(unused)]

use std::{fs::read_to_string, io::Write, ops::Deref};

use clap::{arg, Parser};

const TAB_MULTIPLIER: usize = 1;

impl TokonizerTools for (usize, bool) {
    fn to_option(self) -> Option<usize> {
        let (i, b) = self;
        if !b {
            return Some(i);
        }
        None
    }
}

trait TokonizerTools {
    fn to_option(self) -> Option<usize>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    Char(char),
    Word(String),
    BraceOpen,
    BraceClose,
    BraceSquareClosed,
    BraceSquareOpen,
    WhiteSpace,
    NewLine,
    Comma,
    ParenOpen,
    ParenClose,
    Pipe,
    DoubleQuote,
    SingleQuote,
    SingleQuoteBlock(String),
    DoubleQuoteBlock(String),
    Tab(usize),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '{' => Self::BraceOpen,
            '}' => Self::BraceClose,
            '|' => Self::Pipe,
            '\n' => Self::NewLine,
            ' ' => Self::WhiteSpace,
            '\t' => Self::Tab(0),
            '[' => Self::BraceSquareOpen,
            ']' => Self::BraceSquareClosed,
            ',' => Self::Comma,
            '(' => Self::ParenOpen,
            ')' => Self::ParenClose,
            '"' => Self::DoubleQuote,
            '\'' => Self::SingleQuote,
            _ => Self::Char(value),
        }
    }
}

impl Token {
    fn as_string(&self) -> String {
        match self {
            Token::Char(c) => c.to_string(),
            Token::Word(w) => w.to_string(),
            Token::BraceOpen => "{".to_string(),
            Token::BraceClose => "}".to_string(),
            Token::WhiteSpace => " ".to_string(),
            Token::NewLine => "\n".to_string(),
            Token::Pipe => "|".to_string(),
            Token::BraceSquareOpen => "[".to_string(),
            Token::BraceSquareClosed => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::ParenOpen => "(".to_string(),
            Token::ParenClose => ")".to_string(),
            Token::DoubleQuote => "\"".to_string(),
            Token::SingleQuote => "'".to_string(),
            Token::DoubleQuoteBlock(s) => format!("\"{}\"", s),
            Token::SingleQuoteBlock(s) => format!("\"{}\"", s),
            Token::Tab(n) => vec!["\t"; *n * TAB_MULTIPLIER].join(""),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct AppArgs {
    #[arg(short, long)]
    filename: String,
}

struct Tokonizer {
    index: usize,
    tokens: Vec<Token>,
    stack: Vec<Token>,
}

impl Tokonizer {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            stack: Vec::new(),
        }
    }
    fn next(&mut self) {
        self.index += 1;
    }
    fn prev(&mut self) {
        self.index -= 1;
    }
    fn get(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }
    fn peak_next(&self) -> Option<Token> {
        let index = self.index.overflowing_add(1).to_option();
        self.tokens.get(index?).cloned()
    }
    fn peak_prev(&self) -> Option<Token> {
        let index = self.index.overflowing_sub(1).to_option();
        self.tokens.get(index?).cloned()
    }
    fn next_eq(&self, token: Token) -> bool {
        if self.peak_next() == Some(token) {
            return true;
        }
        false
    }
    fn peak_next_non_whitespace(&self, token: Token) -> Option<Token> {
        let mut index = self.index + 1;
        while let Some(token) = self.tokens.get(index) {
            if token != &Token::WhiteSpace {
                return Some(token.clone())
            }
            index += 1;
        }
        None
    }
    fn prev_eq(&self, token: Token) -> bool {
        if self.peak_prev() == Some(token) {
            return true;
        }
        false
    }
    fn is_eq(&self, token: &Token) -> bool {
        if self.get() == Some(token.clone()) {
            return true;
        }
        false
    }
    fn to_stack(&mut self, token: Token) {
        self.stack.push(token.clone())
    }
    fn take_upto(&mut self, token: Token) -> String {
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

fn gen_bear(buffer: String) -> Vec<Token> {
    let tokens = buffer.chars().map(|e| Token::from(e)).collect::<Vec<_>>();
    let mut word = String::new();
    let mut t = Tokonizer::new(tokens);

    while let Some(token) = t.get() {
        match token {
            Token::Char(_) => word.push_str(&token.as_string()),
            Token::WhiteSpace
            | Token::NewLine
            | Token::Pipe
            | Token::Comma
            | Token::ParenOpen
            | Token::ParenClose
            | Token::BraceSquareOpen
            | Token::BraceSquareClosed => {
                if !word.is_empty() {
                    t.to_stack(Token::Word(word.clone()));
                    word.clear();
                }
                t.to_stack(token);
            }
            Token::DoubleQuote => {
                t.next();
                
                let block = t.take_upto(Token::DoubleQuote);
                t.to_stack(Token::DoubleQuoteBlock(block));
            }
            Token::SingleQuote => {
                t.next();

                let block = t.take_upto(Token::SingleQuote);
                t.to_stack(Token::SingleQuoteBlock(block));
            }

            _ => {
                t.to_stack(token);
            }
        }
        t.next();
    }

    t.stack
        .into_iter()
        .filter(|e| e != &Token::WhiteSpace && e != &Token::Tab(0))
        .collect::<Vec<_>>()
}

#[test]
fn some() {
    let text = r#"
def pm [ ] {
	mut locations = [ item, item2, item3 ]
	let    selection = abs | cselect | as | as | as
	   let a = "this is |  should not touch | | | | "
if $selection != null {
		let a = foobar
		let c = bar
		let b = a | each { | item | item | do_somehting }
		mpv $selection
	}
	
} "#;

    let stack = gen_bear(text.to_string());

    dbg!(stack);
}

fn main() -> Result<(), std::io::Error> {
    let args = AppArgs::parse();

    let filename = args.filename;
    let buffer = read_to_string(&filename)?;

    let mut depth = 0;
    let mut paren_depth = 0;

    let mut t = Tokonizer::new(gen_bear(buffer));

    while let Some(token) = t.get() {
        match token {
            Token::Word(_)
            | Token::Pipe
            | Token::Comma
            | Token::BraceSquareOpen
            | Token::BraceSquareClosed
            | Token::BraceOpen
            | Token::BraceClose => {

                // push current token to token_stack
                t.to_stack(token.clone());

                // add depth for indents
                match token {
                    Token::BraceOpen if paren_depth == 0 => depth += 1,
                    Token::BraceClose if paren_depth == 0 => depth -= 1,
                    Token::ParenOpen => paren_depth += 1,
                    Token::ParenClose => paren_depth -= 1,
                    _ => (),
                }

                // add whitespace if needed
                match t.peak_next() {
                    Some(Token::NewLine) => (),
                    Some(Token::Comma) => (),
                    None | Some(_) => t.to_stack(Token::WhiteSpace),
                };
            }

            Token::NewLine => {
                t.to_stack(token.clone());

                // add indent if next token is a Token::BraceClose it should substract one from depth
                if t.next_eq(Token::BraceClose) {
                    t.to_stack(Token::Tab(depth - 1))
                } else {
                    t.to_stack(Token::Tab(depth))
                }
            }

            _ => t.to_stack(token.clone()),
        }
        t.next();
    }

    let str = t
        .stack
        .iter()
        .map(|e| e.as_string())
        .collect::<Vec<_>>()
        .join("");

    let mut lock = std::io::stdout().lock();
    lock.write_all(str.as_bytes()).unwrap();

    Ok(())
}
