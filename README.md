# Arguments [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a parser for command-line arguments.

## [Documentation][documentation]

## Example

```rust
let arguments = std::env::args(); // foo --no-bar --baz 42 --qux 'To be?'
let arguments = arguments::parse(arguments).unwrap();

println!("Foo: {}", arguments.program);
println!("Bar: {}", arguments.get::<bool>("bar").unwrap());
println!("Baz: {}", arguments.get::<usize>("baz").unwrap());
println!("Qux: {}", arguments.get::<String>("qux").unwrap());
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[documentation]: https://docs.rs/arguments
[status-img]: https://travis-ci.org/stainless-steel/arguments.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/arguments
[version-img]: https://img.shields.io/crates/v/arguments.svg
[version-url]: https://crates.io/crates/arguments
