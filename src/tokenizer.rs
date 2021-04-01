use crate::input_stream::InputStream;

pub enum Token {
    Str(String),   // string literal
    Sym(String),   // symbol
    Int(i32),   // integer literal
    Kw(Keyword),   // language-defined keyword
    Id(String), // user-defined identifier
}

pub enum Keyword {
    CONST,
    LET,
    IF,
    ELSE,
    WHILE,
    WHEN,
    BREAK,
    RETURN,
    CONTINUE,
    TYPEOF,
}
pub struct Tokenizer {
    current: Option<Token>,
    stream: InputStream,
}

fn is_whitespace(s: String) -> bool {
    s == "\t" || s == "\r" || s == "\n" || s == " "
}

impl Tokenizer {
    pub fn new(stream: InputStream) -> Tokenizer {
        Tokenizer {
            current: None,
            stream,
        }
    }

    // read current token without consuming it
    pub fn peek(&mut self) -> Option<Token> {
        if self.current.is_none() && !self.stream.eof() {
            self.advance();
        }

        self.current.take()
    }

    // read current token and move to next
    pub fn consume(&mut self) -> Option<Token> {
        if self.current.is_none() && !self.stream.eof() {
            self.advance();
        }

        let current = self.current.take();
        self.advance();

        current
    }

    // like peek, but panic if expected is not present
    // pub fn expect(&mut self, expected: Token) -> Option<Token>{
    //     if self.current.is_none() && !self.stream.eof() {
    //         self.advance();
    //     }
    // }

    // like consume, but panic if expected is not present
    // pub fn consume_expect(&mut self, expected: Token) -> Option<Token> {
    //
    // }

    // parse one token from input stream
    // eofs silently
    fn advance(&mut self) {
        // clear whitespace
        self.stream.read_while(is_whitespace);

        if self.stream.eof() {
            self.current = None;
            return;
        }

        let c: String = match self.stream.peek() {
            Some(s) => s,
            None => return,
        };

        if c.as_str() == "\"" {
            // string literal
            // eat "
            self.stream.consume();

            // TODO escaped strings
            let val = self.stream.read_while(|s| s != "\"");

            // eat "
            self.stream.consume();

            self.current = Some(Token::Str(val));
        } else {
            // all non-string tokens end on whitespace
            let tokenVal = self.stream.read_while(|s| !is_whitespace(s));
            self.current = match tokenVal.as_str() {
                t if t.parse::<i32>().is_ok() => Some(Token::Int(tokenVal.parse::<i32>().unwrap())),

                "let" => Some(Token::Kw(Keyword::LET)),
                "if" => Some(Token::Kw(Keyword::IF)),
                "else" => Some(Token::Kw(Keyword::ELSE)),
                "while" => Some(Token::Kw(Keyword::WHILE)),
                // TODO valid identifiers only
                t => Some(Token::Id(tokenVal)),
                _ => panic!("Unexpected token {} at {}", tokenVal, self.stream.loc()),
            }
        }

    }
}
