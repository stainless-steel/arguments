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
//! let Arguments { command, options, .. } = Arguments::parse(args).unwrap();
//!
//! println!("Foo: {}", command);
//! println!("Bar: {}", options.get::<bool>("bar").unwrap());
//! println!("Buz: {}", options.get::<String>("buz").unwrap());
//! ```

extern crate options;

/// Command-line arguments.
pub struct Arguments {
    /// The name of the executable.
    pub command: String,
    /// The given options.
    pub options: Options,
    /// The rest of the arguments.
    pub orphans: Vec<String>,

    _private: (),
}

/// Command-line options.
pub use options::Options;

macro_rules! raise(
    ($($arg:tt)*) => (return Err(format!($($arg)*)));
);

impl Arguments {
    /// Parse command-line arguments.
    pub fn parse<I: Iterator<Item=String>>(mut stream: I) -> Result<Arguments, String> {
        let mut arguments = Arguments {
            command: match stream.next() {
                Some(command) => String::from(command),
                _ => raise!("expected at least the name of the executed command"),
            },
            options: Options::new(),
            orphans: Vec::new(),
            _private: (),
        };

        let mut previous: Option<String> = None;
        for chunk in stream {
            if chunk.starts_with("--") {
                if let Some(name) = previous {
                    arguments.options.set(&name, true);
                }
                if chunk.len() == 2 {
                    raise!("expected a name right after “--”");
                }
                previous = Some(String::from(&chunk[2..]));
            } else if let Some(name) = previous {
                arguments.options.set(&name, String::from(chunk));
                previous = None;
            } else {
                arguments.orphans.push(chunk);
            }
        }
        if let Some(name) = previous {
            arguments.options.set(&name, true);
        }

        Ok(arguments)
    }
}

/// Parse command-line arguments.
#[inline]
pub fn parse<I: Iterator<Item=String>>(stream: I) -> Result<Arguments, String> {
    Arguments::parse(stream)
}

#[cfg(test)]
mod tests {
    use Arguments;

    macro_rules! strings(
        ($($str:expr),*) => (
            [$($str),*].iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
    );

    #[test]
    fn orphans() {
        let args = strings!["a", "b", "--c", "d", "e", "--f"];
        let args = args.iter().map(|a| a.to_string());
        let Arguments { orphans, .. } = Arguments::parse(args).unwrap();
        assert_eq!(orphans, strings!["b", "e"]);
    }
}
