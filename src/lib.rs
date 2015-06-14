//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! let args = std::env::args().skip(1);
//! let args = arguments::parse(args).unwrap();
//!
//! match args.get::<String>("input") {
//!     Some(input) => println!("Filename: {}.", input),
//!     _ => println!("Usage: foo --input <filename>"),
//! }
//! ```

extern crate options;

/// A collection of command-line arguments.
pub use options::Options as Arguments;

macro_rules! raise(
    ($($arg:tt)*) => (return Err(format!($($arg)*)));
);

/// Parse command-line arguments.
pub fn parse<I: Iterator<Item=String>>(stream: I) -> Result<Arguments, String> {
    let mut arguments = Arguments::new();
    let mut previous: Option<String> = None;

    for chunk in stream {
        if chunk.starts_with("--") {
            if let Some(name) = previous {
                arguments.set(&name, true);
            }
            if chunk.len() == 2 {
                raise!("expected a name right after “--”");
            }
            previous = Some(String::from(&chunk[2..]));
        } else if let Some(name) = previous {
            arguments.set(&name, String::from(chunk));
            previous = None;
        } else {
            raise!("expected a name starting from “--”, but found “{}”", chunk);
        }
    }
    if let Some(name) = previous {
        arguments.set(&name, true);
    }

    Ok(arguments)
}
