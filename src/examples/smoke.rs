fn main() {
    let text = "Hello, I'm using Claude!";

    let tokens = claude_tokenizer::tokenize(text).unwrap();
    let token_count = claude_tokenizer::count_tokens(text).unwrap();

    println!("Tokens: {:?}", tokens);
    println!("Token count: {}", token_count);
}