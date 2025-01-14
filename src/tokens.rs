
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

pub trait TokonizerTools {
    fn to_option(self) -> Option<usize>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Char(char),
    Word(String),
    BraceOpen,
    BraceClose,
    BraceSquareClosed,
    BraceSquareOpen,
    WhiteSpace,
    NewLine,
    Comma,
    Dolar,
    ParenOpen,
    ParenClose,
    Pipe,
    Hash,
    CommentBlock(String),
    Path(String),
    Slash,
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
            '/' => Self::Slash,
            '$' => Self::Dolar,
            '#' => Self::Hash,
            _ => Self::Char(value),
        }
    }
}

impl Token {
    pub fn as_string(&self) -> String {
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
            Token::Dolar => "$".to_string(),
            Token::SingleQuote => "'".to_string(),
            Token::Slash => "/".to_string(),
            Token::Hash => "#".to_string(),
            Token::CommentBlock(s) => s.to_string(),
            Token::Path(s) => s.to_string(),
            Token::DoubleQuoteBlock(s) => s.to_string(),
            Token::SingleQuoteBlock(s) => s.to_string(),
            Token::Tab(n) => vec!["\t"; *n * TAB_MULTIPLIER].join(""),
        }
    }
}
