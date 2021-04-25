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
            line: 1,
            col: 1,
        })
    }

    // used for testing, takes a string
    pub fn new_from_string(contents: &str) -> InputStream {
        let body: Vec<String> = contents.to_string().graphemes(true).map(|s| s.to_string()).collect();
        InputStream {
            body,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn consume(&mut self) -> String {
        let next: String = self.body[self.pos].clone();

        self.pos += 1;

        if next == "\n" {
            self.line += 1;
            self.col = 1;
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

    pub fn read_while(&mut self, condition: impl Fn(&String) -> bool) -> String {
        let mut output: Vec<String> = Vec::new();

        while !self.eof()
            && condition(
                &self
                    .peek()
                    .expect(format!("Unexpected EOF at {}", self.loc()).as_str()),
            )
        {
            output.push(self.consume());
        }

        output.join("")
    }

    pub fn loc(&self) -> String {
        format!("{}:{}", self.line, self.col)
    }
}
