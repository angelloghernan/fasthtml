mod tokenizer;
mod tokens;

use tokens::Token;
use tokenizer::Tokenizer;

fn main() {
    let html = "<h1><p><body hello><h1><h1><h1><h1 id=\"guide-service\">";
    let mut tokenizer = Tokenizer::new(html);
    tokenizer.tokenize();
    for token in tokenizer.tokens {
        token.print_self(html.as_bytes());
    }
}
