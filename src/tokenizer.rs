use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    OpeningParenthesis,
    ClosingParenthesis,
    Number(String),
    String(String),
    Identifier(String),
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
        if ch == '(' {
            tokens.push(Token::OpeningParenthesis);
            cursor = chars.next();
        } else if ch == ')' {
            tokens.push(Token::ClosingParenthesis);
            cursor = chars.next();
        } else if ch.is_whitespace() {
            cursor = chars.next();
        } else if ch.is_digit(10) {
            let mut value = String::new();

            while let Some(c) = cursor {
                if c.is_digit(10) {
                    value.push(c);
                    cursor = chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Number(value));
        } else if ch == '"' {
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

            tokens.push(Token::String(value));
        } else if ch.is_alphabetic() {
            let mut value = String::new();

            while let Some(c) = cursor {
                if c.is_alphabetic() {
                    value.push(c);
                    cursor = chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Identifier(value));
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
            Token::OpeningParenthesis,
            Token::Identifier("add".to_string()),
            Token::Number("2".to_string()),
            Token::OpeningParenthesis,
            Token::Identifier("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::ClosingParenthesis,
            Token::ClosingParenthesis,
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
