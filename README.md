JSON scanner & parser in rust

Trying to get a better understanding of scanning & parsing in general.

1. First takes a string input and produces a `Vec<Token>` of our `enum Token {...}` type
2. Given a list of tokens, call the `parse` fn and produce a `enum JValue {...}`

References:
- Used this article to get me started on scanning & `char_indicies`: https://petermalmgren.com/token-scanning-with-rust/
