use {crate::tokenizer::*, std::cell::RefCell, std::fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstCallExpression {
    pub name: String,
    pub params: Vec<AbstractSyntaxTree>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractSyntaxTree {
    CallExpression(AstCallExpression),
    NumberLiteral(String),
    StringLiteral(String),
}

#[derive(Debug)]
pub enum ParserError {
    MissingIdentifier,
    ExpectedIdentifier(Token),
    MissingClosingParenthesis,
    UnexpectedClosingParenthesis,
    UnexpectedIdentifier(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parser error: {}",
            match self {
                ParserError::MissingIdentifier => {
                    "missing identifier in CallExpression".to_string()
                }
                ParserError::ExpectedIdentifier(ref token) => {
                    format!("expected identifier in CallExpression, but got {:?}", token)
                }
                ParserError::MissingClosingParenthesis => "missing closing parenthesis".to_string(),
                ParserError::UnexpectedClosingParenthesis => {
                    "unexpected closing parenthesis".to_string()
                }
                ParserError::UnexpectedIdentifier(ref ident) => {
                    format!("unexpected identifier '{}'", ident)
                }
            }
        )
    }
}

fn walk(
    iter: &mut impl Iterator<Item = Token>,
    cursor: &mut RefCell<Option<Token>>,
) -> Result<Option<AbstractSyntaxTree>, ParserError> {
    let token = (*cursor.borrow()).clone();
    if let Some(token) = token {
        match token {
            Token::Number(value) => {
                cursor.replace(iter.next());
                Ok(Some(AbstractSyntaxTree::NumberLiteral(value.clone())))
            }
            Token::String(value) => {
                cursor.replace(iter.next());
                Ok(Some(AbstractSyntaxTree::StringLiteral(value.clone())))
            }
            Token::ClosingParenthesis => Err(ParserError::UnexpectedClosingParenthesis),
            Token::OpeningParenthesis => {
                let ident = iter.next().ok_or_else(|| ParserError::MissingIdentifier)?;

                if let Token::Identifier(ident) = ident {
                    let mut params = Vec::new();

                    cursor.replace(iter.next());
                    loop {
                        match walk(iter, cursor) {
                            Err(ParserError::UnexpectedClosingParenthesis) => break,
                            Ok(Some(token)) => params.push(token),
                            Ok(None) => return Err(ParserError::MissingClosingParenthesis),
                            Err(err) => return Err(err),
                        }
                    }
                    cursor.replace(iter.next());

                    Ok(Some(AbstractSyntaxTree::CallExpression(
                        AstCallExpression {
                            name: ident,
                            params,
                        },
                    )))
                } else {
                    Err(ParserError::ExpectedIdentifier(ident))
                }
            }
            Token::Identifier(ident) => Err(ParserError::UnexpectedIdentifier(ident)),
        }
    } else {
        Ok(None)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<AbstractSyntaxTree>, ParserError> {
    let mut ast = Vec::new();
    let mut iter = tokens.into_iter();
    let mut cursor = RefCell::new(iter.next());

    while let Some(token) = walk(&mut iter, &mut cursor)? {
        ast.push(token);
    }

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use {crate::parser::*, lazy_static::lazy_static};

    lazy_static! {
        static ref TOKENS: Vec<Token> = vec![
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
        static ref AST: [AbstractSyntaxTree; 1] =
            [AbstractSyntaxTree::CallExpression(AstCallExpression {
                name: "add".to_string(),
                params: vec![
                    AbstractSyntaxTree::NumberLiteral("2".to_string()),
                    AbstractSyntaxTree::CallExpression(AstCallExpression {
                        name: "subtract".to_string(),
                        params: vec![
                            AbstractSyntaxTree::NumberLiteral("4".to_string()),
                            AbstractSyntaxTree::NumberLiteral("2".to_string()),
                        ],
                    })
                ],
            }),];
    }

    #[test]
    fn parser_test() {
        let actual = parse(TOKENS.clone()).unwrap();
        assert_eq!(actual.as_slice(), *AST);
    }
}
