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
    pub fn new_from_file(path: &str) -> Result<InputStream, Error> {
        let contents: String = fs::read_to_string(path)?;
        let body: Vec<String> = contents.graphemes(true).map(|s| s.to_string()).collect();
        Ok(InputStream {
            body,
            pos: 0,
            line: 0,
            col: 0,
        })
    }

    // used for testing, takes a vector of segmented unicode graphemes
    pub fn new_from_vec(body: Vec<String>) -> InputStream {
        InputStream {
            body,
            pos: 0,
            line: 0,
            col: 0,
        }
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

    // like consume, but panics if expected doesn't match
    pub fn consume_expect(&mut self, expected: &str) -> String {
        let next = self.consume();

        if next != expected.to_string() {
            panic!("Expected {} but got {} at {}", expected, next, self.loc());
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

    // like peek, but panics if expected doesn't match
    // panics on EOF, to check for EOF use eof()
    pub fn peek_expect(&self, expected: &str) -> Option<String> {
        let next = self.peek().expect(format!("Expected {} but got EOF at {}", expected, self.loc()).as_str());

        if next != expected {
            panic!("Expected {} but got {} at {}", expected, next, self.loc());
        }

        Some(next)
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

    pub fn read_while(&mut self, condition: fn(&String) -> bool) -> String {
        let mut output: String = "".to_string();

        while !self.eof()
            && condition(
                &self
                    .peek()
                    .expect(format!("Unexpected EOF at {}", self.loc()).as_str()),
            )
        {
            output = format!("{}{}", output, self.consume());
        }

        output
    }

    pub fn loc(&self) -> String {
        format!("{}:{}", self.line, self.col)
    }
}
