use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ///////////////////////////////////////////////////////////////////////////
    // single-character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Plus,
    Minus,
    Star,    // *
    Slash,   // /
    Equal,   // =
    Less,    // <
    Greater, // >
    Bang,    // !

    ///////////////////////////////////////////////////////////////////////////
    // two-character tokens
    EqualEqual,   // ==
    LessEqual,    // <=
    GreaterEqual, // >=
    BangEqual,    // !=

    ///////////////////////////////////////////////////////////////////////////
    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    ///////////////////////////////////////////////////////////////////////////
    /// Literals
    StringLiteral(String),
    NumberLiteral(f64),
    Identifier(String),

    // end of file
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: should use the same symbols as in the scanner
            Token::LeftParenthesis => write!(f, "("),
            Token::RightParenthesis => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Semicolon => write!(f, ";"),
            Token::Bang => write!(f, "!"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Equal => write!(f, "="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),

            Token::EqualEqual => write!(f, "=="),
            Token::BangEqual => write!(f, "!="),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),

            // literals
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::NumberLiteral(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),

            // keywords
            Token::And => write!(f, "and"),
            Token::Class => write!(f, "class"),
            Token::Else => write!(f, "else"),
            Token::False => write!(f, "false"),
            Token::Fun => write!(f, "fun"),
            Token::For => write!(f, "for"),
            Token::If => write!(f, "if"),
            Token::Nil => write!(f, "nil"),
            Token::Or => write!(f, "or"),
            Token::Print => write!(f, "print"),
            Token::Return => write!(f, "return"),
            Token::Super => write!(f, "super"),
            Token::This => write!(f, "this"),
            Token::True => write!(f, "true"),
            Token::Var => write!(f, "var"),
            Token::While => write!(f, "while"),

            Token::Eof => write!(f, ""),
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
            "*" => Ok(Token::Star),
            "/" => Ok(Token::Slash),
            "=" => Ok(Token::Equal),
            "==" => Ok(Token::EqualEqual),
            "<" => Ok(Token::Less),
            ">" => Ok(Token::Greater),
            "<=" => Ok(Token::LessEqual),
            ">=" => Ok(Token::GreaterEqual),
            "!" => Ok(Token::Bang),
            "!=" => Ok(Token::BangEqual),
            "(" => Ok(Token::LeftParenthesis),
            ")" => Ok(Token::RightParenthesis),
            "{" => Ok(Token::LeftBrace),
            "}" => Ok(Token::RightBrace),
            "," => Ok(Token::Comma),
            "." => Ok(Token::Dot),
            ";" => Ok(Token::Semicolon),
            "kw:and" => Ok(Token::And),
            "kw:class" => Ok(Token::Class),
            "kw:else" => Ok(Token::Else),
            "kw:false" => Ok(Token::False),
            "kw:fun" => Ok(Token::Fun),
            "kw:for" => Ok(Token::For),
            "kw:if" => Ok(Token::If),
            "kw:nil" => Ok(Token::Nil),
            "kw:or" => Ok(Token::Or),
            "kw:print" => Ok(Token::Print),
            "kw:return" => Ok(Token::Return),
            "kw:super" => Ok(Token::Super),
            "kw:this" => Ok(Token::This),
            "kw:true" => Ok(Token::True),
            "kw:var" => Ok(Token::Var),
            "kw:while" => Ok(Token::While),
            identifier
                if identifier
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_') =>
            {
                Ok(Token::Identifier(identifier.to_string()))
            }
            _ => Err(format!("Unknown token: len: {} : {}", value.len(), value)),
        }
    }
}
