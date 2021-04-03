use crate::input_stream::InputStream;

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment,
    Str(String), // string literal
    Sym(Symbol), // symbol
    Int(i32),    // integer literal
    Kw(Keyword), // language-defined keyword
    Id(String),  // user-defined identifier
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    current: Option<Token>,
    stream: InputStream,
}

fn is_whitespace(s: &String) -> bool {
    s == "\t" || s == "\r" || s == "\n" || s == " "
}

impl Tokenizer {
    pub fn new(stream: InputStream) -> Tokenizer {
        let mut tok = Tokenizer {
            current: None,
            stream,
        };

        // step into first token
        tok.advance();

        tok
    }

    // read current token without consuming it
    pub fn peek(&self) -> &Option<Token> {
        &self.current
    }

    // read current token and move to next
    pub fn consume(&mut self) -> Option<Token> {
        let current = self.current.take();
        self.advance();

        current
    }

    // like peek, but panic if expected is not present
    pub fn expect(&self, expected: &Option<Token>) -> &Option<Token> {
        let t = self.peek();

        if t == expected {
            panic!(
                "Expected {:?} but got {:?} at {}.",
                expected,
                t,
                self.stream.loc()
            );
        }

        t
    }

    // like consume, but panic if expected is not present
    /*
    pub fn consume_expect(&mut self, expected: Token) -> Option<Token> {

    }
    */

    // parse one token from input stream
    // eofs silently
    fn advance(&mut self) {
        // clear whitespace
        self.stream.read_while(is_whitespace);

        if self.stream.eof() {
            self.current = None;
            return;
        }

        self.current = match self.stream.peek().unwrap_or("none".to_string()).as_str() {
            // string literal
            "\"" => {
                // eat "
                self.stream.consume_expect("\"");

                // TODO escaped
                let val = self.stream.read_while(|s| s != "\"");

                // eat "
                self.stream.consume_expect("\"");

                Some(Token::Str(val))
            }
            "/" => {
                // eat /
                self.stream.consume_expect("/");

                match self.stream.peek().expect("Unexpected EOF after /").as_str() {
                    // multi-line comment
                    "*" => {
                        loop {
                            if let (Some(n), Some(m)) = (self.stream.peek(), self.stream.peek_next()) {
                                if n == "*" && m == "/" {
                                    self.stream.consume_expect("*");
                                    self.stream.consume_expect("/");
                                    break Some(Token::Comment)
                                }
                            }

                            if self.stream.eof() {
                                panic!("Unexpected EOF.")
                            }
                           
                            self.stream.consume();
                        }
                    }
                    // single-line comment
                    "/" => {
                        self.stream.read_while(|s| s != "\n");
                        Some(Token::Comment)
                    }
                    "=" => Some(Token::Sym(Symbol::SlashEqual)),
                    // anything else, treat it as division symbol
                    _ => {
                        Some(Token::Sym(Symbol::Slash))
                    }
                }
            }
            t => {
                // TODO error with whole word?
                panic!("Unexpected character {} at {}.", t, self.stream.loc())
            }
        };

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
