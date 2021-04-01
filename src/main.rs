use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    input_stream::InputStream::new("./main.ky")?;
    Ok(())
}

mod input_stream {
    use std::fs;
    use std::io::Error;
    use unicode_segmentation::UnicodeSegmentation;

    pub struct InputStream {
        body: Vec<String>,
        pos: usize,
        line: u32,
        col: u32,
    }

    impl InputStream {
        pub fn new(path: &str) -> Result<InputStream, Error> {
            let contents: String = fs::read_to_string(path)?;
            let body: Vec<String> = contents.graphemes(true).map(|s| s.to_string()).collect();
            Ok(InputStream {
                body,
                pos: 0,
                line: 0,
                col: 0,
            })
        }

        pub fn consume(&mut self) -> String {
            let next: String = self.body[self.pos].clone();

            self.pos += 1;

            if next == "\n" {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            next
        }

        pub fn peek(&self) -> Option<String> {
            if self.eof() {
                None
            } else {
                Some(self.body[self.pos].clone())
            }
        }

        pub fn peek_next(&self) -> Option<String> {
            if self.pos + 1 >= self.body.len() {
                None
            } else {
                Some(self.body[self.pos + 1].clone())
            }
        }

        pub fn eof(&self) -> bool {
            self.pos >= self.body.len()
        }

        pub fn read_while(&mut self, condition: impl Fn(String) -> bool) -> Option<String> {
            let mut output: String = "".to_string();

            while !self.eof() && condition(self.peek()?) {
                output = format!("{}{}", output, self.consume());
            }

            Some(output)
        }

        pub fn loc(&self) -> String {
            format!("{}:{}", self.line, self.col)
        }
    }
}

mod tokenizer {
    use std::io::Error;

    use regex::Regex;

    use lazy_static::lazy_static;

    use crate::input_stream::{self, InputStream};

    pub enum Token {
        Sym(String),   // symbol
        Str(String),   // string literal
        Num(f64),      // number literal
        Kw(Keyword),   // language-defined keyword
        Ident(String), // user-defined identifier
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
        stream: input_stream::InputStream,
    }

    impl Tokenizer {
        pub fn new(path: &str) -> Result<Tokenizer, Error> {
            Ok(Tokenizer {
                current: None,
                stream: InputStream::new(path)?,
            })
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

        // read current token without consuming it
        pub fn peek(&mut self) -> Option<Token> {
            if self.current.is_none() && !self.stream.eof() {
                self.advance();
            }

            self.current.take()
        }

        // like peek, but panic if expected is not present
        pub fn expect(&mut self, expected: String) -> Option<Token>{
            if self.current.is_none() && !self.stream.eof() {
                self.advance();
            }
        }

        // parse one token from input stream
        // eofs silently
        fn advance(&mut self) {
            lazy_static! {
                static ref RE_IS_DIGIT: Regex = Regex::new("").unwrap();
            }

            // clear whitespace
            self.stream
                .read_while(|s| s == "\t" || s == "\r" || s == "\n");

            if self.stream.eof() {
                self.current = None;
                return;
            }

            let c: String = match self.stream.peek() {
                Some(s) => s,
                None => return,
            };

            match c.as_str() {
                "\"" => {
                    // string literal
                    // eat "
                    self.stream.consume();

                    // TODO escapes
                    let val = self.stream.read_while(|s| s != "\"");

                    // eat "
                    self.stream.consume();

                    self.current = Some(Token::Str(val.expect("")));
                },
                x if RE_IS_DIGIT.is_match(x) || x == "-" => {

                },
                _ => panic!("Unexpected token: {} at {}", c, self.stream.loc()),
            }
        }
    }
}
