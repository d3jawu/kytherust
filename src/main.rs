use std::io::Error;

mod input_stream;
mod tokenizer;

fn main() -> Result<(), Error> {
    let is = input_stream::InputStream::new_from_file("./main.ky")?;
    let mut tokenizer = tokenizer::Tokenizer::new(is);

    while tokenizer.peek().is_some() {
        let tok = tokenizer.consume();
    }
    Ok(())
}
