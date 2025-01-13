
#[allow(unused)]
mod tokenizer;
mod tokens;
mod test;

use std::{fs::read_to_string, io::Write, };

use clap::{arg, Parser};
use tokenizer::Tokonizer;
use tokens::Token;

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct AppArgs {
    #[arg(short, long)]
    filename: String,
}

fn gen_tokens(buffer: String) -> Vec<Token> {
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

fn format_buffer(buffer: String) -> String {
    let mut depth = 0;
    let mut paren_depth = 0;

    let mut t = Tokonizer::new(gen_tokens(buffer));

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

    t.stack
        .iter()
        .map(|e| e.as_string())
        .collect::<Vec<_>>()
        .join("")
}





fn main() -> Result<(), std::io::Error> {
    let args = AppArgs::parse();

    let filename = args.filename;
    let buffer = read_to_string(&filename)?;


    let new_buffer = format_buffer(buffer);

    let mut lock = std::io::stdout().lock();
    lock.write_all(new_buffer.as_bytes()).unwrap();

    Ok(())
}
