mod tokenizer;
mod tokens;

use tokens::Token;

fn main() {
    println!("Size of token: {}", std::mem::size_of::<Token>());
}
