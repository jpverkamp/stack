use std::io::BufRead;

use regex::Regex;
use substring::Substring;

use crate::types::Token;

/// Tokenizes a stream of characters into a vector of tokens.
pub fn tokenize(reader: impl BufRead) -> Vec<Token> {
    let mut tokens = vec![];
    let token_patterns = vec![
        // single characters
        r"[\{}()\[\]]",
        // numbers
        r"-?\d+(\.\d*)?(/\d+(\.\d*)?)",
        // strings
        "\"(\\.|[^\"])\"",
        // alphanumeric identifiers
        r"[a-zA-Z0-9_]+",
        // symbolic identifier
        // todo: quotes?
        r"[^a-zA-Z0-9\s\{}()\[\]]+",
    ];
    let token_regex = Regex::new(format!("^({})", token_patterns.join("|")).as_str()).unwrap();
    let whitespace_regex = Regex::new(r"^\s+").unwrap();

    // Scan the input line by line tracking rows
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut line = line.as_str();
        let mut column = 0;

        // Within a row, scan for tokens character by character
        loop {
            if let Some(c) = whitespace_regex.captures(line) {
                line = line.substring(c[0].len(), line.len());
                column += c[0].len();
            }

            if let Some(c) = token_regex.captures(line) {
                tokens.push(Token {
                    row,
                    column,
                    token: c[0].to_string(),
                });
                line = line.substring(c[0].len(), line.len());
                column = c[0].len();
            } else if line.len() != 0 {
                panic!("no token found at {row}:{column} = {line:?}");
            }

            if line.len() == 0 {
                break;
            }
        }
    }

    tokens
}
