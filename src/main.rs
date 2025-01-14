mod test;
#[allow(unused)]
mod tokenizer;
mod tokens;

use std::{fs::read_to_string, io::Write};

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
    let mut t = Tokonizer::new(tokens);

    while let Some(token) = t.get() {
        match token {
            Token::Char(_) => t.to_temp(token.as_string()),
            Token::WhiteSpace
            | Token::NewLine
            | Token::Pipe
            | Token::Comma
            | Token::ParenOpen
            | Token::ParenClose
            | Token::BraceSquareOpen
            | Token::BraceSquareClosed => {
                // move word to stack
                t.temp_to_word();

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
                let block = t.take_upto(
                    &[Token::WhiteSpace, Token::BraceClose ],
                    true,
                );
                t.to_stack(Token::Path(format!("{}{}", t.temp, block)));
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
        
        // push current to stack
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
        match token {
            Token::Word(_) => match t.peak_next_non_whitespace() {
                Some(Token::Comma) => (),
                Some(Token::NewLine) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::BraceOpen => match t.peak_next_non_whitespace() {
                Some(Token::BraceOpen)
                | Some(Token::BraceSquareOpen)
                | Some(Token::ParenOpen)
                | Some(Token::BraceClose) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::BraceClose | Token::BraceSquareClosed => match t.peak_next_non_whitespace() {
                Some(Token::BraceClose)
                | Some(Token::BraceSquareClosed)
                | Some(Token::ParenClose) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::BraceSquareOpen => match t.peak_next_non_whitespace() {
                Some(Token::BraceOpen)
                | Some(Token::BraceSquareOpen)
                | Some(Token::ParenOpen)
                | Some(Token::BraceSquareClosed) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::ParenOpen => match t.peak_next_non_whitespace() {
                Some(Token::BraceOpen) | Some(Token::BraceSquareOpen) | Some(Token::ParenClose) => {
                    ()
                }
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::ParenClose => match t.peak_next_non_whitespace() {
                Some(Token::BraceClose)
                | Some(Token::BraceSquareClosed)
                | Some(Token::ParenClose)
                | Some(Token::Comma)
                | Some(Token::Slash) => (),
                _ => t.to_stack(Token::WhiteSpace),
            },

            Token::Dolar => (),
            Token::Hash => (),
            Token::CommentBlock(_) => (),
            Token::Slash => (),
            _ => t.to_stack(Token::WhiteSpace),
        }

        // add indentation
        match token {
            Token::NewLine => {
                // add indent if next token is a Token::BraceClose it should substract one from depth
                if t.next_eq(Token::BraceClose) {
                    t.to_stack(Token::Tab(depth - 1))
                } else {
                    t.to_stack(Token::Tab(depth))
                }
            }
            _ => (),
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
