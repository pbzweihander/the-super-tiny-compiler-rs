use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Parenthesis,
    Number,
    String,
    Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug)]
pub struct TokenizerError {
    pub character: char,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tokenizer error: cannot understand character {}",
            self.character
        )
    }
}

pub fn tokenizer(code: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = vec![];
    let mut chars = code.chars();
    let mut cursor = chars.next();

    while let Some(ch) = cursor {
        if ch == '(' || ch == ')' {
            // Parenthesis
            tokens.push(Token {
                kind: TokenKind::Parenthesis,
                value: ch.to_string(),
            });

            cursor = chars.next();
        } else if ch.is_whitespace() {
            // Whitespace
            cursor = chars.next();
        } else if ch.is_digit(10) {
            // Number literal
            let mut value = String::new();

            while let Some(c) = cursor {
                if c.is_digit(10) {
                    value.push(c);
                    cursor = chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token {
                kind: TokenKind::Number,
                value,
            });
        } else if ch == '"' {
            // String literal
            let mut value = String::new();

            while {
                cursor = chars.next();
                if let Some(c) = cursor {
                    if c != '"' {
                        value.push(c);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } {}
            cursor = chars.next();

            tokens.push(Token {
                kind: TokenKind::String,
                value,
            });
        } else if ch.is_alphabetic() {
            // Name
            let mut value = String::new();

            while let Some(c) = cursor {
                if c.is_alphabetic() {
                    value.push(c);
                    cursor = chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token {
                kind: TokenKind::Name,
                value,
            });
        } else {
            return Err(TokenizerError { character: ch });
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use {crate::tokenizer::*, lazy_static::lazy_static};

    lazy_static! {
        static ref INPUT: &'static str = "(add 2 (subtract 4 2))";
        static ref TOKENS: [Token; 9] = [
            Token {
                kind: TokenKind::Parenthesis,
                value: "(".to_string(),
            },
            Token {
                kind: TokenKind::Name,
                value: "add".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                value: "2".to_string(),
            },
            Token {
                kind: TokenKind::Parenthesis,
                value: "(".to_string(),
            },
            Token {
                kind: TokenKind::Name,
                value: "subtract".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                value: "4".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                value: "2".to_string(),
            },
            Token {
                kind: TokenKind::Parenthesis,
                value: ")".to_string(),
            },
            Token {
                kind: TokenKind::Parenthesis,
                value: ")".to_string(),
            },
        ];
    }

    #[test]
    fn tokenizer_test() {
        let actual = tokenizer(&INPUT).unwrap();
        assert_eq!(
            actual.as_slice(),
            *TOKENS,
            "Tokenizer test failed: {:?} != {:?}",
            actual,
            *TOKENS,
        );
    }
}
