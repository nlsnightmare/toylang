mod char_tokenizer;
mod comment_tokenizer;
mod identifier_tokenizer;
mod keyword_tokenizer;
mod number_tokenizer;
mod string_tokenizer;
mod whitespace_tokenizer;

pub use char_tokenizer::CharTokenizer;
pub use comment_tokenizer::CommentTokenizer;
pub use identifier_tokenizer::IdentifierTokenizer;
pub use keyword_tokenizer::KeywordTokenizer;
pub use number_tokenizer::NumberTokenizer;
use regex::Regex;
pub use string_tokenizer::StringTokenizer;
pub use whitespace_tokenizer::WhitespaceTokenizer;

use super::{lexer::Position, TokenWrapper};

pub trait Tokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper>;
}

fn consume_regex(view: &str, regex: &Regex) -> Option<(String, usize)> {
    let groups = regex.captures(&view)?;

    let first = groups.get(0)?;
    let captured = groups.get(1)?;

    let full_captured = first.end() - first.start();

    let text = view[captured.start()..captured.end()].to_owned();

    Some((text, full_captured))
}

