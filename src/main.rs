mod tokenizer;
mod tokens;

use tokens::Token;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("<h1><p><body hello><h1><h1><h1>");
    tokenizer.tokenize();
    for token in tokenizer.tokens {
        println!("Token: {:?}", token);
    }
}
