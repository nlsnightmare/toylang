use std::path::Path;

use parser::Parser;
use runtime::Runtime;
use tokenizer::Tokenizer;

mod parser;
mod runtime;
mod tokenizer;

fn main() {
    let argument = std::env::args().nth(1).expect("Please provide a filename");
    let filepath = Path::new(&argument);

    if !filepath.exists() {
        panic!("File not found: {}", argument);
    }

    let contents = std::fs::read_to_string(filepath)
        .expect("Unable to read file")
        .to_string();

    let tokens = Tokenizer::tokenize(contents);
    let ast = Parser::new(tokens)
        .parse()
        .expect("Unable to parse program!");

    Runtime::new().run(ast);
}
