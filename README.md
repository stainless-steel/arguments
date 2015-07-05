# Arguments [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for command-line arguments.

## [Documentation][doc]

## Example

```rust
let arguments = std::env::args(); // foo --no-bar --baz 42 --qux 'To be?'
let arguments = arguments::parse(arguments).unwrap();

println!("Foo: {}", arguments.program);
println!("Bar: {}", arguments.get::<bool>("bar").unwrap());
println!("Baz: {}", arguments.get::<usize>("baz").unwrap());
println!("Qux: {}", arguments.get::<String>("qux").unwrap());
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/arguments.svg
[version-url]: https://crates.io/crates/arguments
[status-img]: https://travis-ci.org/stainless-steel/arguments.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/arguments
[doc]: https://stainless-steel.github.io/arguments
