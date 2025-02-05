
const TAB_MULTIPLIER: usize = 2;

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
    Number(String),
    Tilda,
    Att,
    BraceOpen,
    BraceClose,
    BraceSquareClosed,
    BraceSquareOpen,
    WhiteSpace,
    NewLine,
    Comma,
    Equals,
    Dolar,
    ParenOpen,
    ParenClose,
    Exc,
    LessThen,
    MoreThen,
    Pipe,
    Hash,
    Colon,
    CommentBlock(String),
    Path(String),
    Dash,
    Tag(String),
    TagLong(String),
    AttSomething(String),
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
            '=' => Self::Equals,
            '!' => Self::Exc,
            '>' => Self::MoreThen,
            '<' => Self::LessThen,
            '#' => Self::Hash,
            ':' => Self::Colon,
            '@' => Self::Att,
            '~' => Self::Tilda,
            '-' => Self::Dash,
            _ => Self::Char(value),
        }
    }
}

impl Token {
    pub fn as_string(&self) -> String {
        match self {
            Token::Char(c) => c.to_string(),
            Token::Word(w) => w.to_string(),
            Token::Number(w) => w.to_string(),
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
            Token::Equals => "=".to_string(),
            Token::MoreThen => ">".to_string(),
            Token::LessThen=> "<".to_string(),
            Token::Exc=> "!".to_string(),
            Token::Colon=> ":".to_string(),
            Token::Tilda=> "~".to_string(),
            Token::Att => "@".to_string(),
            Token::Dash => "-".to_string(),
            Token::Tag(s) => s.to_string(),
            Token::TagLong(s) => s.to_string(),
            Token::AttSomething(s) => s.to_string(),
            Token::CommentBlock(s) => s.to_string(),
            Token::Path(s) => s.to_string(),
            Token::DoubleQuoteBlock(s) => s.to_string(),
            Token::SingleQuoteBlock(s) => s.to_string(),
            Token::Tab(n) => vec!["\t"; *n * TAB_MULTIPLIER].join(""),
        }
    }
}
