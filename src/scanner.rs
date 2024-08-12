#[derive(Debug)]
pub enum Token {
    OpenBrace,
    ClosingBrace,
    OpenBracket,
    ClosingBracket,
    Comma,
    Colon,
    StringLiteral(String),
    Number(f64),
    True,
    False,
    Null,
}

pub type Tokens = Vec<Token>;

pub fn scan(input: String) -> Result<Tokens, String> {
    let mut tokens: Tokens = Vec::new();
    let mut char_indices = input.char_indices().peekable();

    while let Some((_pos, ch)) = char_indices.next() {
        // skip whitespace's (\t, \r, ' ' etc)
        if ch.is_whitespace() {
            continue;
        };

        let token = match ch {
            '{' => Token::OpenBrace,
            '}' => Token::ClosingBrace,
            '[' => Token::OpenBracket,
            ']' => Token::ClosingBracket,
            ',' => Token::Comma,
            ':' => Token::Colon,
            // number
            '0'..='9' | '-' => {
                let mut number_string = ch.to_string();

                while let Some((_pos, next_ch)) = char_indices.peek() {
                    if next_ch.is_ascii_digit() || *next_ch == '.' {
                        number_string.push(*next_ch);
                        char_indices.next();
                    } else {
                        break;
                    }
                }

                match number_string.parse::<f64>() {
                    Ok(num) => Token::Number(num),
                    Err(_) => {
                        return Err(format!(
                            "Unable to parse string number to float64, got: {:?}",
                            number_string
                        ))
                    }
                }
            }
            // string
            '"' => {
                let mut last_matched = None;
                let mut escaped = false;

                let string: String = char_indices
                    .by_ref()
                    .take_while(|(_pos, char)| {
                        if escaped {
                            escaped = false;
                            true
                        } else if *char == '\\' {
                            escaped = true;
                            true
                        } else {
                            last_matched = Some(*char);
                            *char != '"'
                        }
                    })
                    .map(|(_pos, char)| char)
                    .collect();

                match last_matched {
                    Some('"') => Token::StringLiteral(string),
                    _ => {
                        return Err(format!(
                            "Unterminated string literal, got: {:?}",
                            last_matched
                        ))
                    }
                }
            }
            // identifiers (true, false, null)
            'a'..='z' => {
                let mut identifier = ch.to_string();

                while let Some((_pos, next_ch)) = char_indices.peek() {
                    if next_ch.is_alphanumeric() {
                        identifier.push(*next_ch);
                        char_indices.next();
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "true" => Token::True,
                    "false" => Token::False,
                    "null" => Token::Null,
                    _ => {
                        return Err(format!(
                            "Expected identifier (true, false, null), got: {:?}",
                            identifier
                        ))
                    }
                }
            }
            _ => return Err(format!("Invalid json, got: {:?}", ch)),
        };

        tokens.push(token);
    }

    Ok(tokens)
}
