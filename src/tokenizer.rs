use crate::input_stream::InputStream;

pub enum Token {
    Str(String), // string literal
    Sym(String), // symbol
    Int(i32),    // integer literal
    Kw(Keyword), // language-defined keyword
    Id(String),  // user-defined identifier
}

pub enum Keyword {
    Const,
    Let,
    If,
    Else,
    While,
    When,
    Break,
    Return,
    Typeof,
    Continue,
    Import,
    Export,
}

pub enum Symbol {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,

    Bar,
    And,
    BarBar,
    AndAnd,

    EqualEqual,
    EqualEqualEqual,
    BangEqual,
    BangEqualEqual,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Bang,

    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Semicolon,
    Colon,
}
pub struct Tokenizer {
    stream: InputStream,
}

fn is_whitespace(s: &String) -> bool {
    s == "\t" || s == "\r" || s == "\n" || s == " "
}

impl Tokenizer {
    pub fn new(stream: InputStream) -> Tokenizer {
        Tokenizer {
            stream,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    // parse one token from input stream
    fn next(&mut self) -> Option<Token> {
        if self.stream.eof() {
            return None;
        }

        // clear whitespace
        self.stream.read_while(is_whitespace);

        let next_char: String = self.stream.peek()?;

        match next_char.as_str() {
            // string literal
            "\"" => {
                // eat "
                self.stream.consume();

                // TODO escaped strings
                let val = self.stream.read_while(|s| s != "\"");

                // eat "
                self.stream.consume();

                Some(Token::Str(val))
            }
            // comment
            "/" => {
                // eat /
                self.stream.consume();

                None
            }
            t => {
                panic!("Invalid token {} at {}.", t, self.stream.loc())
            }
        }

        // // all non-string tokens end on whitespace or separator
        // let tokenVal = self
        //     .stream
        //     .read_while(|s| !is_whitespace(&s) && s != "," && s != ";");
        // self.current = match tokenVal.as_str() {
        //     t if t.parse::<i32>().is_ok() => Some(Token::Int(tokenVal.parse::<i32>().unwrap())),

        //     "let" => Some(Token::Kw(Keyword::Let)),
        //     "if" => Some(Token::Kw(Keyword::If)),
        //     "else" => Some(Token::Kw(Keyword::Else)),
        //     "while" => Some(Token::Kw(Keyword::While)),
        //     // TODO valid identifiers only
        //     t => Some(Token::Id(tokenVal)),
        //     _ => panic!("Unexpected token {} at {}", tokenVal, self.stream.loc()),
        // }
    }
}