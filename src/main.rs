mod parser;
mod scanner;

// sample input:
// let input = String::from("{\"key\": [\"value1\", \"value2\"], \"flag\": true, \"nullable\": null, \"number\": 12.46, \"nested\": {\"nested_key\": []}}");
//
fn main() -> Result<(), String> {
    let json_bytes = include_bytes!("./sample.json");
    let json_file = String::from_utf8_lossy(json_bytes).to_string();

    let scanned_tokens = scanner::scan(json_file);

    match scanned_tokens {
        Ok(vec_tokens) => {
            let mut tokens = vec_tokens.iter().peekable();
            let json = parser::parse(&mut tokens);
            println!("json result: {:#?}", json);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
