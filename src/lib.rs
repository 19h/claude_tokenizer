use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

use anyhow::Result;
use tokenizers::InputSequence;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::{EncodeInput, Tokenizer};

pub fn get_tokenizer() -> Tokenizer {
    let tokenizer_data = include_bytes!("claude-v1-tokenization.json");

    Tokenizer::from_bytes(tokenizer_data)
        .unwrap()
}

pub fn tokenize(text: &str) -> Result<Vec<(u32, String)>> {
    let tokenizer = get_tokenizer();

    let val = EncodeInput::Single(InputSequence::Raw(text.into()));

    let encoded_text =
        tokenizer.encode(
            val,
            false,
        );

    match encoded_text {
        Ok(encoded_text) =>
            Ok(
                encoded_text
                    .get_ids()
                    .iter()
                    .zip(
                        encoded_text
                            .get_tokens()
                            .iter()
                            .cloned(),
                    )
                    .map(|(id, token)| (*id, token.to_string()))
                    .collect()
            ),
        Err(err) =>
            Err(
                anyhow::Error::msg(
                    err.to_string(),
                )
            )
    }
}

pub fn count_tokens(text: &str) -> Result<usize> {
    let tokenizer = get_tokenizer();

    let val = EncodeInput::Single(InputSequence::Raw(text.into()));

    let encoded_text =
        tokenizer.encode(
            val,
            false,
        );

    match encoded_text {
        Ok(encoded_text) => {
            Ok(
                encoded_text.len()
            )
        }
        Err(err) =>
            Err(
                anyhow::Error::msg(
                    err.to_string(),
                )
            ),
    }
}
