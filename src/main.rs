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

        pub fn next(&mut self) -> String {
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
                output = format!("{}{}", output, self.next());
            }

            Some(output)
        }
    }
}

mod tokenizer {
    use std::io::Error;

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

        fn read_while(&mut self, condition: impl Fn(String) -> bool) {}

        // read current token and move to next
        pub fn consume(&mut self) -> Option<Token> {}

        // read current token without consuming it
        pub fn peek(&mut self) -> Option<Token> {
            
        }

        // parse one token from input stream into self.current
        fn next(&mut self) {
            let raw: String = self.stream.next();
        }
    }
}
