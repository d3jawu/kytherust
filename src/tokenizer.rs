use crate::input_stream::InputStream;

#[derive(Debug, PartialEq)]
pub enum Token {
    Str(String), // string literal
    Sym(Symbol), // symbol
    Int(i32),    // integer literal
    Double(f64), // FP literal
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

// faster than a regex
fn is_digit(s: &str) -> bool {
    ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&s)
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
            // symbols
            "+" => sym_or_sym_and!("=", Plus, PlusEqual),
            "-" => sym_or_sym_and!("=", Minus, MinusEqual),
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
            t if is_digit(t) => {
                let mut output: Vec<String> = vec![t.to_string()];
                let mut has_dec = false;

                while let Some(s) = self.stream.peek() {
                    if is_digit(&s) || s == "." {
                        if s == "." {
                            if has_dec {
                                panic!("Unexpected '.' at {}", self.stream.loc())
                            }

                            has_dec = true;
                        }

                        self.stream.consume_expect(&s);
                        output.push(s);
                    } else {
                        break;
                    }
                }

                let result = output.join("");

                if has_dec {
                    Some(Token::Double(result.parse::<f64>().unwrap_or_else(|_| {
                        panic!("Could not parse number: {}", result)
                    })))
                } else {
                    Some(Token::Int(result.parse::<i32>().unwrap_or_else(|_| {
                        panic!("Could not parse integer: {}", result)
                    })))
                }
            }
            t => {
                // read as whole word

                panic!("Unexpected character {} at {}.", t, self.stream.loc())
            }
        };
    }
}
