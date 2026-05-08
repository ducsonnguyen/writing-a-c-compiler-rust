use regex::Regex;
use std::fmt;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i64),
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
    DoubleHyphen,
}

#[derive(Debug)]
pub struct LexError(String);

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lexer error: {}", self.0)
    }
}

impl std::error::Error for LexError {}

struct TokenPattern {
    regex: Regex,
    kind: TokenKind,
}

enum TokenKind {
    Int,
    Void,
    Return,
    Identifier,
    Constant,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
    DoubleHyphen,
}

static PATTERNS: LazyLock<Vec<TokenPattern>> = LazyLock::new(|| {
    vec![
        // Order matters: first match wins. Keywords must come before identifier.
        TokenPattern { regex: Regex::new(r"^int\b").unwrap(), kind: TokenKind::Int },
        TokenPattern { regex: Regex::new(r"^void\b").unwrap(), kind: TokenKind::Void },
        TokenPattern { regex: Regex::new(r"^return\b").unwrap(), kind: TokenKind::Return },
        TokenPattern { regex: Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(), kind: TokenKind::Identifier },
        TokenPattern { regex: Regex::new(r"^[0-9]+\b").unwrap(), kind: TokenKind::Constant },
        TokenPattern { regex: Regex::new(r"^\(").unwrap(), kind: TokenKind::OpenParen },
        TokenPattern { regex: Regex::new(r"^\)").unwrap(), kind: TokenKind::CloseParen },
        TokenPattern { regex: Regex::new(r"^\{").unwrap(), kind: TokenKind::OpenBrace },
        TokenPattern { regex: Regex::new(r"^\}").unwrap(), kind: TokenKind::CloseBrace },
        TokenPattern { regex: Regex::new(r"^;").unwrap(), kind: TokenKind::Semicolon },
        TokenPattern { regex: Regex::new(r"^~").unwrap(), kind: TokenKind::Tilde },
        // `--` must come before `-` since first match wins
        TokenPattern { regex: Regex::new(r"^--").unwrap(), kind: TokenKind::DoubleHyphen },
        TokenPattern { regex: Regex::new(r"^-").unwrap(), kind: TokenKind::Hyphen },
    ]
});

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            break;
        }

        // First matching pattern wins (keywords are listed before identifier)
        let mut matched_token: Option<(usize, &TokenKind)> = None;
        for pattern in PATTERNS.iter() {
            if let Some(m) = pattern.regex.find(remaining) {
                matched_token = Some((m.len(), &pattern.kind));
                break;
            }
        }

        match matched_token {
            Some((len, kind)) => {
                let matched = &remaining[..len];
                let token = match kind {
                    TokenKind::Int => Token::Int,
                    TokenKind::Void => Token::Void,
                    TokenKind::Return => Token::Return,
                    TokenKind::Identifier => Token::Identifier(matched.to_string()),
                    TokenKind::Constant => {
                        let value = matched.parse::<i64>().map_err(|e| {
                            LexError(format!("invalid integer constant '{matched}': {e}"))
                        })?;
                        Token::Constant(value)
                    }
                    TokenKind::OpenParen => Token::OpenParen,
                    TokenKind::CloseParen => Token::CloseParen,
                    TokenKind::OpenBrace => Token::OpenBrace,
                    TokenKind::CloseBrace => Token::CloseBrace,
                    TokenKind::Semicolon => Token::Semicolon,
                    TokenKind::Tilde => Token::Tilde,
                    TokenKind::Hyphen => Token::Hyphen,
                    TokenKind::DoubleHyphen => Token::DoubleHyphen,
                };
                tokens.push(token);
                remaining = &remaining[len..];
            }
            None => {
                let bad_char = remaining.chars().next().unwrap();
                return Err(LexError(format!("unexpected character '{bad_char}'")));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program() {
        let tokens = lex("int main(void) { return 2; }").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".to_string()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Constant(2),
                Token::Semicolon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_invalid_character() {
        let result = lex("@");
        assert!(result.is_err());
    }

    #[test]
    fn test_identifier_starting_with_keyword() {
        let tokens = lex("return_value").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("return_value".to_string())]);
    }
}
