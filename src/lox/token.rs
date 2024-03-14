use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    ///////////////////////////////////////////////////////////////////////////
    // single-character tokens
    Plus,
    Minus,
    Times, // *
    Divide,
    Assign,  // =
    Less,    // <
    Greater, // >

    ///////////////////////////////////////////////////////////////////////////
    // two-character tokens
    Equal,        // ==
    LessEqual,    // <=
    GreaterEqual, // >=

    ///////////////////////////////////////////////////////////////////////////
    // keywords
    Print,

    ///////////////////////////////////////////////////////////////////////////
    /// Literals
    StringLiteral(String),
    NumberLiteral(f64),

    // end of file
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: should use the same symbols as in the scanner
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Print => write!(f, "print"),
            Token::Assign => write!(f, "="),
            Token::Equal => write!(f, "=="),
            Token::Eof => write!(f, ""),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::NumberLiteral(n) => write!(f, "{}", n),
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // string literal
        if value.starts_with("\"") {
            println!("value: {} : {}", value, value.len());
            if !(value.chars().count() > 1) {
                return Err(format!(
                    "String literal len must be greater than 1, got {} : {}",
                    value.len(),
                    value
                ));
            }

            if !value.ends_with("\"") {
                return Err(format!("String literal not closed: {}", value));
            }

            let s = value[1..value.len() - 1].to_string();

            println!("PROCESSED LITERAL: {}", s);
            return Ok(Token::StringLiteral(s));
        }

        // number literal
        if value.parse::<f64>().is_ok() {
            let n = value.parse::<f64>().unwrap();
            return Ok(Token::NumberLiteral(n));
        }

        match value {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Times),
            "/" => Ok(Token::Divide),
            "print" => Ok(Token::Print),
            "=" => Ok(Token::Assign),
            "==" => Ok(Token::Equal),
            "<" => Ok(Token::Less),
            ">" => Ok(Token::Greater),
            "<=" => Ok(Token::LessEqual),
            ">=" => Ok(Token::GreaterEqual),
            _ => Err(format!("Unknown token: len: {} : {}", value.len(), value)),
        }
    }
}
