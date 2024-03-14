use super::Token;

pub struct Scanner {
    source: String,
}

struct ScanInfo {
    line: u64,
    line_offset: u64,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source: source }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        if !self.source.is_ascii() {
            return Err("Source is not ASCII".to_string());
        }

        let mut char_iterator = self.source.chars();
        let mut scan_info = ScanInfo {
            line: 0,
            line_offset: 0,
        };

        while let Some(c) = char_iterator.nth(0) {
            Scanner::match_root(c, &mut char_iterator, &mut tokens, &mut scan_info);
        }

        tokens.push(Token::Eof);

        return Ok(tokens);
    }

    #[inline(always)]
    fn match_root(
        c: char,
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        scan_info: &mut ScanInfo,
    ) {
        match c {
            '+' => {
                tokens.push(Token::Plus);
            }
            '-' => {
                tokens.push(Token::Minus);
            }
            '*' => {
                tokens.push(Token::Times);
            }
            '/' => {
                Scanner::match_divide(char_iterator, tokens, scan_info);
            }
            '=' => {
                Scanner::match_assign(char_iterator, tokens, scan_info);
            }
            '<' => {
                Scanner::match_less(char_iterator, tokens, scan_info);
            }
            '>' => {
                Scanner::match_greater(char_iterator, tokens, scan_info);
            }
            '"' => {
                Scanner::match_string_literal(char_iterator, tokens, scan_info);
            }
            '\n' => {
                scan_info.line += 1;
                scan_info.line_offset = 0;
            }
            ' ' => {
                scan_info.line_offset = 0;
            }
            digit if digit.is_ascii_digit() => {
                Scanner::match_number_literal(digit, char_iterator, tokens, scan_info);
            }
            alpha if alpha.is_ascii_alphabetic() => {
                Scanner::match_identifier(alpha, char_iterator, tokens, scan_info);
            }
            other => {
                // match identifier, then convert to keyword, identifier or literal
            }
        }
    }

    #[inline(always)]
    fn match_assign(
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        scan_info: &mut ScanInfo,
    ) {
        match char_iterator.nth(0) {
            Some('=') => {
                tokens.push(Token::Equal);
            }
            Some(other) => {
                tokens.push(Token::Assign);
                Scanner::match_root(other, char_iterator, tokens, scan_info);
            }
            None => {
                tokens.push(Token::Assign);
            }
        }
    }

    #[inline(always)]
    fn match_less(
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        scan_info: &mut ScanInfo,
    ) {
        match char_iterator.nth(0) {
            Some('=') => {
                tokens.push(Token::LessEqual);
            }
            Some(other) => {
                tokens.push(Token::Less);
                Scanner::match_root(other, char_iterator, tokens, scan_info);
            }
            None => {
                tokens.push(Token::Less);
            }
        }
    }

    #[inline(always)]
    fn match_greater(
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        scan_info: &mut ScanInfo,
    ) {
        match char_iterator.nth(0) {
            Some('=') => {
                tokens.push(Token::GreaterEqual);
            }
            Some(other) => {
                tokens.push(Token::Greater);
                Scanner::match_root(other, char_iterator, tokens, scan_info);
            }
            None => {
                tokens.push(Token::Greater);
            }
        }
    }

    #[inline(always)]
    fn match_divide(
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        scan_info: &mut ScanInfo,
    ) {
        match char_iterator.nth(0) {
            Some('/') => {
                // line comment
                Scanner::match_line_comment(char_iterator, scan_info)
            }
            Some(other) => {
                tokens.push(Token::Divide);
                Scanner::match_root(other, char_iterator, tokens, scan_info);
            }
            None => {
                tokens.push(Token::Divide);
            }
        }
    }

    #[inline(always)]
    fn match_line_comment(char_iterator: &mut std::str::Chars, scan_info: &mut ScanInfo) {
        // consume characters until the end of the line is reached, or no more chars are available
        while let Some(c) = char_iterator.nth(0) {
            match c {
                '\n' => {
                    scan_info.line += 1;
                    scan_info.line_offset = 0;
                    break;
                }
                _ => {}
            }
        }
    }

    #[inline(always)]
    fn match_string_literal(
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        _scan_info: &mut ScanInfo,
    ) {
        let mut str_buffer = String::with_capacity(128);
        // consume characters until the end of the string is reached, or no more chars are available
        while let Some(c) = char_iterator.nth(0) {
            match c {
                '"' => {
                    // end of string
                    tokens.push(Token::StringLiteral(str_buffer));
                    break;
                }
                other => {
                    str_buffer.push(other);
                }
            }
        }

        // FIXME: end of file reached, but string is not closed, return error
    }

    #[inline(always)]
    fn match_number_literal(
        first: char,
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        _scan_info: &mut ScanInfo,
    ) {
        let mut number_buffer = String::with_capacity(32);
        number_buffer.push(first);

        let mut decimal_point_scanned = false;

        // consume characters until the end of the number is reached, or no more chars are available
        while let Some(c) = char_iterator.nth(0) {
            match c {
                digit if digit.is_ascii_digit() => {
                    number_buffer.push(digit);
                }
                '.' => {
                    if decimal_point_scanned {
                        // TODO: return error
                    }

                    // decimal point
                    number_buffer.push('.');
                    decimal_point_scanned = true;
                }
                other => {
                    // end of number
                    match number_buffer.parse::<f64>() {
                        Ok(n) => tokens.push(Token::NumberLiteral(n)),
                        Err(_e) => {
                            // TODO: return error
                        }
                    }

                    Scanner::match_root(other, char_iterator, tokens, _scan_info);

                    // FIXME: This is ugly. Needed to avoid the code bellow for EOF
                    return;
                }
            }
        }

        // EOF reached, try to parse the number
        match number_buffer.parse::<f64>() {
            Ok(n) => tokens.push(Token::NumberLiteral(n)),
            Err(_e) => {
                // TODO: return error
            }
        }
    }

    #[inline(always)]
    fn match_identifier(
        first: char,
        char_iterator: &mut std::str::Chars,
        tokens: &mut Vec<Token>,
        _scan_info: &mut ScanInfo,
    ) {
        let mut identifier_buffer = String::with_capacity(64);
        identifier_buffer.push(first);

        // consume characters until the end of the identifier is reached, or no more chars are available
        while let Some(c) = char_iterator.nth(0) {
            match c {
                alpha_num if alpha_num.is_ascii_alphanumeric() => {
                    identifier_buffer.push(c);
                }
                other => {
                    match identifier_buffer.as_str() {
                        "and" => tokens.push(Token::And),
                        "class" => tokens.push(Token::Class),
                        "else" => tokens.push(Token::Else),
                        "false" => tokens.push(Token::False),
                        "fun" => tokens.push(Token::Fun),
                        "for" => tokens.push(Token::For),
                        "if" => tokens.push(Token::If),
                        "nil" => tokens.push(Token::Nil),
                        "or" => tokens.push(Token::Or),
                        "print" => tokens.push(Token::Print),
                        "return" => tokens.push(Token::Return),
                        "super" => tokens.push(Token::Super),
                        "this" => tokens.push(Token::This),
                        "true" => tokens.push(Token::True),
                        "var" => tokens.push(Token::Var),
                        "while" => tokens.push(Token::While),
                        other => tokens.push(Token::Identifier(other.to_string())),
                    }

                    Scanner::match_root(other, char_iterator, tokens, _scan_info);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use std::path::PathBuf;

    use rstest::*;

    use super::*;

    #[test]
    fn test_scan_tokens() {
        ///////////////////////////////////////////////////////////////////////
        // Given a source string
        let source = String::from("+-*/=*-");

        ///////////////////////////////////////////////////////////////////////
        // When the source is scanned
        let mut scanner = Scanner::new(source);
        let tokens_result = scanner.scan_tokens();

        ///////////////////////////////////////////////////////////////////////
        // Then the result is OK
        assert!(tokens_result.is_ok());

        // And the tokens are as expected
        let tokens = tokens_result.unwrap();

        let expected_tokens = vec![
            Token::Plus,
            Token::Minus,
            Token::Times,
            Token::Divide,
            Token::Assign,
            Token::Times,
            Token::Minus,
            Token::Eof,
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (computed, expected) in zip(&tokens, &expected_tokens) {
            assert_eq!(computed, expected);
        }
    }

    #[rstest]
    fn test_from_file(#[files("test-data/scanner/**/")] base_path: PathBuf) -> Result<(), String> {
        ///////////////////////////////////////////////////////////////////////
        // Given the content of a source file
        let input_source =
            std::fs::read_to_string(base_path.join("input.txt")).map_err(|e| e.to_string())?;

        // and its corresponding expected tokens
        let expected_tokens = read_expected_tokens(base_path.join("expected.txt"))?;

        ///////////////////////////////////////////////////////////////////////
        // When the source is scanned
        let mut scanner = Scanner::new(input_source);
        let computed_tokens = scanner.scan_tokens()?;

        ///////////////////////////////////////////////////////////////////////

        for (i, (computed, expected)) in zip(&computed_tokens, &expected_tokens).enumerate() {
            assert_eq!(computed, expected, "Token mismatch at index {}", i);
        }

        // Then the resulting tokens match the expected tokens
        assert_eq!(
            computed_tokens.len(),
            expected_tokens.len(),
            "Token vector length mismatch (computed, expected)"
        );

        Ok(())
    }

    fn read_expected_tokens(path: PathBuf) -> Result<Vec<Token>, String> {
        // raw file content
        let expecteed_tokens_source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        // split by line and attempt to convert to tokens
        let expected_tokens_result: Vec<_> = expecteed_tokens_source
            .split("\n")
            .map(Token::try_from)
            .collect();

        let mut tokens = Vec::<Token>::new();
        for e in expected_tokens_result {
            match e {
                Ok(t) => tokens.push(t),
                Err(e) => return Err(e),
            }
        }

        tokens.push(Token::Eof);

        return Ok(tokens);
    }
}
