# Arguments [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for command-line arguments.

## [Documentation][doc]

## Example

```rust
use arguments::Arguments;

let args = std::env::args(); // foo --bar --buz qux
let Arguments { program, options, .. } = arguments::parse(args).unwrap();

println!("Foo: {}", program);
println!("Bar: {}", options.get::<bool>("bar").unwrap());
println!("Buz: {}", options.get::<String>("buz").unwrap());
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
