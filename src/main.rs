use std::io::Error;

mod input_stream;
mod tokenizer;
mod parser;
mod vm;

fn main() -> Result<(), Error> {
    let is = input_stream::InputStream::new_from_file("./main.ky")?;
    let tokenizer = tokenizer::Tokenizer::new(is);
    let mut parser = parser::Parser::new(tokenizer);

    let program = parser.parse();

    for node in program {
        println!("{:#?}", node);
    }

    Ok(())
}
