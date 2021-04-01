use std::fs;
use std::io::Error;

mod input_stream;
mod tokenizer;

fn main() -> Result<(), Error> {
    let is = input_stream::InputStream::new_from_file("./main.ky")?;
    let tokenizer = tokenizer::Tokenizer::new(is);
    Ok(())
}
