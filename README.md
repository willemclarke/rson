JSON scanner & parser in rust

Trying to get a better understanding of scanning & parsing in general.

1. First takes a string input and produces a `Vec<Token>` of our `enum Token {...}` type
2. Given a list of tokens, call the `parse` fn and produce a `enum JValue {...}`

The following input: `String::from("{\"key\": [\"value1\", \"value2\"], \"flag\": true, \"nullable\": null, \"number\": 12.46, \"nested\": {\"nested_key\": []}}")`

Produces:

```rust
Ok(JObject({
    "nested": JObject({"nested_key": JArray([])}),
    "number": JNumber(12.46),
    "flag": JBool(true),
    "key": JArray([JString("value1"), JString("value2")]),
    "nullable": JNull
}))
```

References:
- Used this article to get me started on scanning & `char_indicies`: https://petermalmgren.com/token-scanning-with-rust/
