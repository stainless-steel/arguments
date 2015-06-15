//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! use arguments::Arguments;
//!
//! let args = std::env::args(); // foo --bar --buz qux
//! # let args = vec!["foo", "--bar", "--buz", "quz"];
//! # let args = args.iter().map(|a| a.to_string());
//! let Arguments { program, options, .. } = arguments::parse(args).unwrap();
//!
//! println!("Foo: {}", program);
//! println!("Bar: {}", options.get::<bool>("bar").unwrap());
//! println!("Buz: {}", options.get::<String>("buz").unwrap());
//! ```

extern crate options;

macro_rules! raise(
    ($($arg:tt)*) => (return Err(format!($($arg)*)));
);

/// Command-line arguments.
pub struct Arguments {
    /// The name of the executable.
    pub program: String,
    /// The options given.
    pub options: Options,
    /// Unclassified arguments.
    pub orphans: Vec<String>,
}

/// Command-line options.
pub use options::Options;

mod parser;

pub use parser::Parser;

/// Parse command-line arguments.
#[inline]
pub fn parse<I: Iterator<Item=String>>(stream: I) -> Result<Arguments, String> {
    Parser::new().parse(stream)
}
