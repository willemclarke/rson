mod parser;
mod scanner;

fn main() -> Result<(), String> {
    let input = String::from("{\"key\": [\"value1\", \"value2\"], \"flag\": true, \"nullable\": null, \"number\": 12.46.5, \"nested\": {\"nested_key\": []}}");
    let scanned_tokens = scanner::scan(input);

    match scanned_tokens {
        Ok(vec_tokens) => {
            let mut tokens = vec_tokens.iter().peekable();
            let json = parser::parse(&mut tokens);
            println!("json result: {:?}", json);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
