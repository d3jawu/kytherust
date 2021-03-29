use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    input_stream::InputStream::new("./main.ky")?;
    Ok(())
}

pub enum Token {
    Sym,   // symbol
    Str,   // string literal
    Num,   // number literal
    Kw,    // language-defined keyword
    Ident, // user-defined identifier
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

        pub fn peek(&self) -> String {
            self.body[self.pos].clone()
        }

        pub fn peek_next(&self) -> Option<String> {
            self.body[self.pos + 1].clone()?
        }

        pub fn eof(&self) -> bool {
            self.pos == self.body.len()
        }
    }
}

mod tokenizer {
    struct Tokenizer {}

    impl Tokenizer {}
}
