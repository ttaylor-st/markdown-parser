# markdown-parser

A markdown parser written in Rust, usable as a library or as a WebAssembly module.

## Usage

```rust
use markdown_parser::parse_markdown;

fn main() {
    let markdown = "# Hello, world!";
    let html = parse_markdown(markdown);
    println!("{}", html);
}
```

It can also be compiled into WebAssembly and used in the browser:

```typescript
// TODO: Add WASM usage example
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for more details.
