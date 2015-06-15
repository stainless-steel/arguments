//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! let args = std::env::args(); // foo --bar --buz 42 --qux 'To be?'
//! # let args = vec!["foo", "--bar", "--buz", "42", "--qux", "To be?"];
//! # let args = args.iter().map(|a| a.to_string());
//! let args = arguments::parse(args).unwrap();
//!
//! println!("Foo: {}", args.program);
//! println!("Bar: {}", args.get::<bool>("bar").unwrap());
//! println!("Buz: {}", args.get::<usize>("buz").unwrap());
//! println!("Qux: {}", args.get::<String>("qux").unwrap());
//! ```

extern crate options;

macro_rules! raise(
    ($($arg:tt)*) => (return Err(format!($($arg)*)));
);

/// Command-line options.
pub use options::Options;

mod arguments;
mod parser;

pub use arguments::Arguments;
pub use parser::Parser;

/// Parse command-line arguments.
#[inline]
pub fn parse<I: Iterator<Item=String>>(stream: I) -> Result<Arguments, String> {
    Parser::new().parse(stream)
}
