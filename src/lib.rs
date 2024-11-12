//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! // foo --no-bar --baz 42 --baz 69 --qux "Hello, world!"
//! let arguments = std::env::args();
//! # let arguments = vec![
//! #     "foo",
//! #     "--no-bar",
//! #     "--baz", "42",
//! #     "--baz", "69",
//! #     "--qux", "Hello, world!",
//! # ];
//! # let arguments = arguments.iter().map(|argument| argument.to_string());
//! let arguments = arguments::parse(arguments).unwrap();
//!
//! assert_eq!(arguments.program, "foo");
//! assert_eq!(arguments.get::<bool>("bar").unwrap(), false);
//! assert_eq!(arguments.get::<usize>("baz").unwrap(), 69);
//! assert_eq!(arguments.get_all::<usize>("baz").unwrap(), &[42, 69]);
//! assert_eq!(arguments.get::<String>("qux").unwrap(), "Hello, world!");
//! ```

macro_rules! raise(
    ($message:expr) => (
        return Err(std::io::Error::other($message))
    );
);

mod arguments;

pub use options::Options;

pub use crate::arguments::Arguments;

/// Parse command-line arguments.
#[inline]
pub fn parse<I: Iterator<Item = String>>(stream: I) -> std::io::Result<Arguments> {
    Arguments::parse(stream)
}
