use crate::input_stream::InputStream;

#[derive(Debug, PartialEq)]
pub enum Token {
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
    // EqualEqualEqual,
    BangEqual,
    // BangEqualEqual,
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

// is this faster than a regex checking for a digit?
fn is_number(s: &str) -> bool {
    ["0", "1", "2", "3", "4", "5" ].contains(&s)
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
    fn advance(&mut self) {
        // clear non-tokens (whitespace and comments)
        loop {
            match self.stream.peek() {
                // possibly a comment
                Some(c) if c == "/" => {
                    match self.stream.peek_next().unwrap_or("".to_string()).as_str() {
                        // multi-line comment
                        "*" => loop {
                            if let (Some(n), Some(m)) =
                                (self.stream.peek(), self.stream.peek_next())
                            {
                                if n == "*" && m == "/" {
                                    self.stream.consume_expect("*");
                                    self.stream.consume_expect("/");
                                    break;
                                }
                            }

                            if self.stream.eof() {
                                panic!("Unexpected EOF.")
                            }

                            self.stream.consume();
                        },
                        // single-line comment
                        "/" => {
                            self.stream.read_while(|s| s != "\n");
                            self.stream.consume_expect("\n");
                        }
                        // anything else, treat it as a / token and continue to token parsing
                        _ => break,
                    };
                }
                // whitespace
                Some(c) if is_whitespace(&c) => {
                    self.stream.read_while(is_whitespace);
                }
                _ => break, // no non-tokens ahead, move on to actual parsing
            }
        }

        // check for EOF
        if self.stream.eof() {
            panic!("Unexpected EOF.")
        }

        macro_rules! some_sym_tok {
            ($sym:ident) => {
                Some(Token::Sym(Symbol::$sym))
            };
        }

        macro_rules! sym_or_sym_and {
            ($and:expr, $sym_name:ident, $sym_and_name:ident) => {
                if let Some(c) = self.stream.peek() {
                    match c.as_str() {
                        $and => {
                            self.stream.consume_expect($and);
                            some_sym_tok!($sym_and_name)
                        }
                        _ => some_sym_tok!($sym_name), // if anything else, treat it as just sym
                    }
                } else {
                    panic!("Unexpected EOF.")
                }
            };
        }

        // at this point, there is definitely a token ahead
        self.current = match self.stream.consume().as_str() {
            // string literal
            "\"" => {
                // TODO escaped
                let val = self.stream.read_while(|s| s != "\"");

                // eat "
                self.stream.consume_expect("\"");

                Some(Token::Str(val))
            }
            // number
            t if is_number(t) || t == "-" => {
                // TODO
                None
            }
            // symbols
            "+" => sym_or_sym_and!("=", Plus, PlusEqual),
            // minus is handled with number
            "*" => sym_or_sym_and!("=", Star, StarEqual),
            "/" => sym_or_sym_and!("=", Slash, SlashEqual),
            "%" => sym_or_sym_and!("=", Percent, PercentEqual),

            "|" => sym_or_sym_and!("|", Bar, BarBar),
            "&" => sym_or_sym_and!("&", And, AndAnd),

            "=" => sym_or_sym_and!("=", Equal, EqualEqual),
            "!" => sym_or_sym_and!("=", Bang, BangEqual),

            "<" => sym_or_sym_and!("=", Less, LessEqual),
            ">" => sym_or_sym_and!("=", Greater, GreaterEqual),

            "." => some_sym_tok!(Dot),
            "(" => some_sym_tok!(LeftParen),
            ")" => some_sym_tok!(RightParen),
            "{" => some_sym_tok!(LeftBrace),
            "}" => some_sym_tok!(RightBrace),
            "[" => some_sym_tok!(LeftBracket),
            "]" => some_sym_tok!(RightBracket),

            "," => some_sym_tok!(Comma),
            ";" => some_sym_tok!(Semicolon),
            ":" => some_sym_tok!(Colon),
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
