#[cfg(test)]
mod test;

#[allow(unused)]
mod tokenizer;
mod tokens;

use std::{fs::read_to_string, io::Write};

use clap::{arg, Parser};
use tokenizer::Tokonizer;
use tokens::{Token, TokonizerTools};

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct AppArgs {
    #[arg(short, long)]
    filename: String,
}

fn gen_tokens(buffer: String) -> Vec<Token> {
    let tokens = buffer.chars().map(Token::from).collect::<Vec<_>>();
    let mut t = Tokonizer::new(tokens);

    while let Some(token) = t.get() {
        match token {
            Token::Char(_) => t.to_temp(token.as_string()),
            Token::WhiteSpace
            | Token::NewLine
            | Token::Pipe
            | Token::Comma
            | Token::Equals
            | Token::ParenOpen
            | Token::MoreThen
            | Token::LessThen
            | Token::Exc
            | Token::ParenClose
            | Token::BraceOpen
            | Token::BraceClose
            | Token::BraceSquareOpen
            | Token::BraceSquareClosed
            | Token::Tilda
            | Token::Colon => {
                // move word to stack
                t.temp_to_word_or_number();

                // push current to stack
                t.to_stack(token);
            }
            Token::DoubleQuote => {
                // take tokens upto DoubleQuote and add them to the stack
                let block = t.take_upto(&[Token::DoubleQuote], true);

                t.to_stack(Token::DoubleQuoteBlock(block));
            }
            Token::SingleQuote => {
                // take tokens upto SingleQuote and add them to the stack
                let block = t.take_upto(&[Token::SingleQuote], true);

                t.to_stack(Token::SingleQuoteBlock(block));
            }
            // if a slash is found it means there is a path.. taking the path untill next white
            // space or brace close or paren close
            Token::Slash => {
                let block = t
                    .take_upto(&[Token::WhiteSpace, Token::BraceClose], true)
                    .trim_end()
                    .to_string();

                t.to_stack(Token::Path(format!("{}{}", t.temp, block)));
                t.temp.clear();
            }
            Token::Att=> {
                let block = t
                    .take_upto(&[Token::NewLine], false)
                    .trim_end()
                    .to_string();

                t.to_stack(Token::AttSomething(format!("{}{}", t.temp, block)));
                t.temp.clear();
            }

            // Commant block
            Token::Hash => {
                // If line starts with a hash its a commant block.. walking upto new line and but
                // the line in a commant_block token. formatting is ignored on this line, commant
                // indent is still applied
                let block = t.take_upto(&[Token::NewLine], false);

                t.to_stack(Token::CommentBlock(block));
            }

            _ => {
                // push current to stack
                t.to_stack(token);
            }
        }
        t.next();
    }

    t.temp_to_word_or_number();
    t.stack
        .into_iter()
        .filter(|e| e != &Token::WhiteSpace && e != &Token::Tab(0))
        .collect::<Vec<_>>()
}

fn add_whitespace(t: &mut Tokonizer, token: &Token) {
    match token {
        Token::Word(_) => match t.peak_next_non_whitespace() {
            Some(Token::Comma) | Some(Token::NewLine) | Some(Token::Colon) | Some(Token::MoreThen) | None => (),
            Some(Token::LessThen) => match t.peak_prev_non_whitespace() {
                Some(Token::Colon) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },
            _ => t.to_stack(Token::WhiteSpace),
        },
        Token::Number(_) => match t.peak_next_non_whitespace() {
            Some(Token::NewLine) => (),
            _ => t.to_stack(Token::WhiteSpace),
        }

        Token::BraceOpen => match t.peak_next_non_whitespace() {
            Some(Token::BraceOpen)
            | Some(Token::BraceSquareOpen)
            | Some(Token::ParenOpen)
            | Some(Token::BraceClose)
            | Some(Token::NewLine)
            | None => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        Token::BraceClose | Token::BraceSquareClosed => match t.peak_next_non_whitespace() {
            Some(Token::BraceClose)
            | Some(Token::BraceSquareClosed)
            | Some(Token::ParenClose)
            | Some(Token::NewLine)
            | Some(Token::Comma)
            | None => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        Token::BraceSquareOpen => match t.peak_next_non_whitespace() {
            Some(Token::BraceOpen)
            | Some(Token::BraceSquareOpen)
            | Some(Token::ParenOpen)
            | Some(Token::BraceSquareClosed)
            | Some(Token::NewLine)
            | None => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        Token::ParenOpen => match t.peak_next_non_whitespace() {
            Some(Token::BraceOpen)
            | Some(Token::BraceSquareOpen)
            | Some(Token::ParenClose)
            | Some(Token::NewLine)
            | None => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        Token::ParenClose => match t.peak_next_non_whitespace() {
            Some(Token::BraceClose)
            | Some(Token::BraceSquareClosed)
            | Some(Token::ParenClose)
            | Some(Token::Comma)
            | Some(Token::Slash)
            | Some(Token::NewLine)
            | None => (),
            _ => t.to_stack(Token::WhiteSpace),
        },
        Token::DoubleQuoteBlock(_) | Token::SingleQuoteBlock(_) => {
            match t.peak_next_non_whitespace() {
                Some(Token::NewLine) | None => (),
                _ => t.to_stack(Token::WhiteSpace),
            }
        }
        Token::Equals | Token::MoreThen | Token::Exc => match t.peak_next_non_whitespace() {
            Some(Token::Equals) | Some(Token::Tilda) => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        Token::Comma => match t.peak_next_non_whitespace() {
            Some(Token::NewLine) => (),
            _ => t.to_stack(Token::WhiteSpace),
        },
        Token::Colon | Token::Tilda => match t.peak_next_non_whitespace() {
            Some(Token::Path(_)) => (),
            _ => t.to_stack(Token::WhiteSpace),
        },
        Token::LessThen => match t.peak_next_non_whitespace() {
            Some(Token::Equals) | Some(Token::Word(_)) => (),
            _ => t.to_stack(Token::WhiteSpace),
        },

        // dont add white space to these tokens
        Token::NewLine | Token::Dolar | Token::Hash | Token::CommentBlock(_) | Token::Slash => (),
        _ => t.to_stack(Token::WhiteSpace),
    }
}

fn add_depth(token: &Token, depth: &mut usize) {
    match token {
        Token::BraceOpen | Token::BraceSquareOpen | Token::ParenOpen => *depth += 1,
        Token::BraceClose | Token::BraceSquareClosed | Token::ParenClose => {
            *depth = depth.overflowing_sub(1).to_option().unwrap_or_default()
        }
        _ => (),
    }
}

fn add_indent(t: &mut Tokonizer, token: &Token, depth: &mut usize) {
    match token {
        Token::NewLine => {
            // add indent if next token is a Token::BraceClose it should substract one from depth
            if t.next_eq(Token::BraceClose)
                | t.next_eq(Token::BraceSquareClosed)
                | t.next_eq(Token::ParenClose)
            {
                t.to_stack(Token::Tab(
                    depth.overflowing_sub(1).to_option().unwrap_or_default(),
                ));
            } else {
                t.to_stack(Token::Tab(depth.clone()));
            }
        }
        _ => (),
    }
}

impl ToString for Tokonizer {
    fn to_string(&self) -> String {
        self.stack
            .iter()
            .map(|e| e.as_string())
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

pub fn format_buffer(buffer: String) -> String {
    let mut depth: usize = 0;
    let mut t = Tokonizer::new(gen_tokens(buffer));

    while let Some(token) = t.get() {
        t.to_stack(token.clone());
        add_depth(&token, &mut depth);
        add_whitespace(&mut t, &token);
        add_indent(&mut t, &token, &mut depth);
        t.next();
    }

    t.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let args = AppArgs::parse();

    let filename = args.filename;
    let buffer = read_to_string(&filename)?;

    let new_buffer = format_buffer(buffer);

    let mut lock = std::io::stdout().lock();
    lock.write_all(new_buffer.as_bytes())?;

    Ok(())
}
