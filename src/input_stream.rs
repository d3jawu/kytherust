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

    pub fn read_while(&mut self, condition: fn(String) -> bool) -> String {
        let mut output: String = "".to_string();

        while !self.eof()
            && condition(
                self.peek()
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
