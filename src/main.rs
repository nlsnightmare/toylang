use std::path::Path;

mod lexer;
mod parser;
mod runtime;

use lexer::Lexer;
use parser::Parser;
use runtime::Runtime;

fn main() {
    let argument = std::env::args().nth(1).expect("Please provide a filename");
    let filepath = Path::new(&argument);

    if !filepath.exists() {
        panic!("File not found: {}", argument);
    }

    let contents = std::fs::read_to_string(filepath)
        .expect("Unable to read file")
        .to_string();

    let tokens = Lexer::tokenize(contents);
    let ast = Parser::new(tokens)
        .parse()
        .expect("Unable to parse program!");

    Runtime::new().run(ast);
}
