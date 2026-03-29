# rsjson

A zero dependency JSON parser written in Rust.

## Overview

rsjson is a lightweight JSON parser that tokenizes and parses JSON strings into a structured `JsonValue` type. It includes pretty printing with indentation and provides line/column information in error messages.

This project is under **active development**.

## Features

- Zero external dependencies
- Recursive descent parser with a hand-written lexer

## Usage

```rust
use rsjson::parse;

let input = r#"{"name": "prudhvi", "age": 25, "skills": ["rust", "python"]}"#;

match parse(input) {
    Ok(value) => println!("{value}"),
    Err(err) => eprintln!("{err}"),
}
```

Output:

```json
{
    "name": "prudhvi",
    "age": 25,
    "skills": [
        "rust",
        "python"
    ]
}
```

## WASM Playground

The repo includes a `rsjson-wasm` crate that compiles the parser to WebAssembly. There's a browser-based playground where you can paste JSON and get a syntax-highlighted, pretty printed output.

### Building the WASM package

```sh
cargo install wasm-pack
cd rsjson-wasm
wasm-pack build --target web
```

Then copy the output to the docs folder for deployment:

```sh
cp pkg/rsjson_wasm.js ../docs/
cp pkg/rsjson_wasm_bg.wasm ../docs/
```

Or use the Makefile at the project root:

```sh
make wasm
```

## Project Structure

```
rsjson/
├── src/
│   ├── lib.rs        # Public API, JsonValue type, pretty printing
│   ├── lexer.rs      # Tokenizer
│   └── parser.rs     # Recursive descent parser
├── rsjson-wasm/
│   ├── src/lib.rs    # WASM bindings
│   └── web/          # Dev HTML page
├── docs/             # Deployed playground (GitHub Pages / Cloudflare)
└── Makefile
```

## Running Tests

```sh
cargo test
```

## License

MIT