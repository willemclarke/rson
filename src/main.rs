use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
mod scanner;

#[derive(Debug)]
enum JValue {
    JString(String),
    JNumber(f64),
    JBool(bool),
    JNull,
    JObject(HashMap<String, JValue>),
    JArray(Vec<JValue>),
}

fn parse(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    match tokens.peek() {
        Some(scanner::Token::True) | Some(scanner::Token::False) => parse_bool(tokens),
        Some(scanner::Token::Null) => parse_null(tokens),
        Some(scanner::Token::Number(_)) => parse_number(tokens),
        Some(scanner::Token::StringLiteral(_)) => parse_string(tokens),
        Some(scanner::Token::OpenBrace) => parse_object(tokens),
        Some(scanner::Token::OpenBracket) => parse_array(tokens),
        _ => parse_bool(tokens),
    }
}

fn parse_bool(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    match tokens.next() {
        Some(scanner::Token::True) => Ok(JValue::JBool(true)),
        Some(scanner::Token::False) => Ok(JValue::JBool(false)),
        _ => Err(format!("Expected true/false got: {:?}", tokens.next())),
    }
}

fn parse_null(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    match tokens.next() {
        Some(scanner::Token::Null) => Ok(JValue::JNull),
        _ => Err(format!("Expected null got: {:?}", tokens.next())),
    }
}

fn parse_number(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    match tokens.next() {
        Some(scanner::Token::Number(num)) => Ok(JValue::JNumber(*num)),
        _ => Err(format!("Expected number got: {:?}", tokens.next())),
    }
}

fn parse_string(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    match tokens.next() {
        Some(scanner::Token::StringLiteral(string)) => Ok(JValue::JString(string.clone())),
        _ => Err(format!("Expected number got: {:?}", tokens.next())),
    }
}

fn parse_object(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    let mut dict: HashMap<String, JValue> = HashMap::new();

    // consume the "{"
    tokens.next();

    loop {
        match tokens.peek() {
            // if next token is a `}`, return the dict
            Some(scanner::Token::ClosingBrace) => {
                // consume the `}`
                tokens.next();
                return Ok(JValue::JObject(dict));
            }

            // if next token is string literal, need to check if its followed by a `:` to be a valid object key.
            // if it is, parse the following value and insert into dict
            Some(scanner::Token::StringLiteral(key)) => {
                // consume the string `key`
                tokens.next();

                // if the `key` is followed bya colon, parse the value
                if let Some(scanner::Token::Colon) = tokens.next() {
                    // TODO: error handle `parse` call
                    let value = parse(tokens)?;
                    dict.insert(key.clone(), value);

                    // if the next character is a `,` advance the iterator
                    // if the next character is a `}`, advance the iterator and return the dict
                    match tokens.peek() {
                        Some(scanner::Token::Comma) => {
                            tokens.next(); // Consume ','
                        }
                        Some(scanner::Token::ClosingBrace) => {
                            tokens.next();
                            return Ok(JValue::JObject(dict));
                        }
                        _ => return Err("Expected comma or closing brace".to_string()),
                    }
                } else {
                    return Err(format!("Object key: {:?} was not followed by a colon", key));
                }
            }
            _ => return Err(format!("Invalid object declaration")),
        }
    }
}

fn parse_array(tokens: &mut Peekable<Iter<scanner::Token>>) -> Result<JValue, String> {
    let mut vector: Vec<JValue> = Vec::new();

    // Consume the `[`
    tokens.next();

    loop {
        match tokens.peek() {
            Some(scanner::Token::ClosingBracket) => {
                // Consume the `]`
                tokens.next();
                return Ok(JValue::JArray(vector));
            }
            Some(_) => {
                // Parse the next value
                let value = parse(tokens)?;
                vector.push(value);

                match tokens.peek() {
                    Some(scanner::Token::Comma) => {
                        tokens.next(); // Consume the `,`
                    }
                    Some(scanner::Token::ClosingBracket) => {
                        // consume `]`
                        tokens.next();
                        return Ok(JValue::JArray(vector));
                    }
                    _ => {
                        return Err(format!(
                            "Array item was not separated by a comma, or there was no closing `]`"
                        ));
                    }
                }
            }
            None => return Err(format!("Unexpected end of input while parsing array")),
        }
    }
}

fn main() {
    let input = String::from("{\"key\": [\"value1\", \"value2\"], \"flag\": true, \"nullable\": null, \"nested\": {\"nested_key\": []}}");

    let scanned_tokens = scanner::scan(input).expect("Failed to scan tokens");
    let mut tokens = scanned_tokens.iter().peekable();

    let json = parse(&mut tokens);
    println!("json result: {:?}", json)
}
