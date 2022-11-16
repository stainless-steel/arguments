# Arguments [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a parser for command-line arguments.

## Example

```rust
// foo --no-bar --baz 42 --baz 69 --qux "Hello, world!"
let arguments = std::env::args();
let arguments = arguments::parse(arguments).unwrap();

assert_eq!(arguments.program, "foo");
assert_eq!(arguments.get::<bool>("bar").unwrap(), false);
assert_eq!(arguments.get::<usize>("baz").unwrap(), 69);
assert_eq!(arguments.get_all::<usize>("baz").unwrap(), &[42, 69]);
assert_eq!(arguments.get::<String>("qux").unwrap(), "Hello, world!");
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/arguments/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/arguments/actions/workflows/build.yml
[documentation-img]: https://docs.rs/arguments/badge.svg
[documentation-url]: https://docs.rs/arguments
[package-img]: https://img.shields.io/crates/v/arguments.svg
[package-url]: https://crates.io/crates/arguments
