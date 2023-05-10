# Claude Tokenizer 🤖🪄

![Crates.io](https://img.shields.io/crates/v/claude_tokenizer.svg)
![Crates.io](https://img.shields.io/crates/d/claude_tokenizer.svg)

Welcome to the **Claude Tokenizer Crate** for Anthropic's AI assistant, Claude! This Rust crate provides a tokenizer for Claude, as well as a convenience method called `count_tokens` that allows you to pass in an arbitrary string and count its tokens.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Example](#example)
- [Disclaimer](#disclaimer)
- [License](#license)

## Features

- Tokenizer for Anthropic's AI assistant, Claude.
- `tokenize` convenience method to tokenize an arbitrary string.
- `count_tokens` convenience method to count tokens in an arbitrary string.
- JSON tokenization file support.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
claude_tokenizer = "0.1.0"
```

Then, run `cargo build` to build your project.

## Usage

### Example

To use the `claude_tokenizer` crate, add the following to your Rust file:

```rust
fn main() {
    let text = "Hello, I'm using Claude!";

    let tokens = claude_tokenizer::tokenize(text).unwrap();
    let token_count = claude_tokenizer::count_tokens(text).unwrap();

    println!("Tokens: {:?}", tokens);
    println!("Token count: {}", token_count);
}

// Tokens: [(10002, "Hello"), (16, ","), (373, "ĠI"), (2338, "'m"), (1626, "Ġusing"), (37918, "ĠClaude"), (5, "!")]
// Token count: 7
```

## Disclaimer

It's currently unknown whether it's permitted to embed the tokenization JSON file provided by Claude. We're doing so for now, but if there are any issues, please let us know.

**Note to Anthropic employees:** If you're reading this and embedding the tokenization JSON file is not allowed, please contact me via email to resolve the issue.

## License

The Claude Tokenizer Crate is licensed under the MIT License.

MIT License

Copyright (c) 2023 Kenan Sulayman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.