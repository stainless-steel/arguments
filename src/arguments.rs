use std::io::Result;
use std::str::FromStr;

use options::Options;

/// Command-line arguments.
#[derive(Debug)]
pub struct Arguments {
    /// The name of the executable.
    pub program: String,
    /// Raw options (all strings).
    pub options: Options,
    /// Unclassified arguments.
    pub orphans: Vec<String>,
}

impl Arguments {
    /// Parse command-line arguments.
    pub fn parse<I: Iterator<Item = String>>(mut stream: I) -> Result<Self> {
        let mut arguments = Self {
            program: match stream.next() {
                Some(ref program) if !program.starts_with("--") => String::from(&program[..]),
                _ => raise!("expected a name as the first argument"),
            },
            options: Default::default(),
            orphans: Default::default(),
        };

        let mut previous: Option<String> = None;

        macro_rules! set(
            ($name:expr) => (
                if let Some(name) = $name.strip_prefix("no-") {
                    if name.is_empty() {
                        raise!("expected a name right after “--no-”");
                    }
                    set!(name, "false");
                } else {
                    set!($name, "true");
                }
            );
            ($name:expr, $value:expr) => (
                match arguments.options.get_mut::<Vec<_>>($name) {
                    Some(array) => {
                        array.push(String::from($value));
                    }
                    _ => {
                        arguments.options.set($name, vec![String::from($value)]);
                    }
                }
            );
        );

        for chunk in stream {
            if let Some(name) = chunk.strip_prefix("--") {
                if let Some(ref name) = previous {
                    set!(name);
                }
                if name.is_empty() {
                    raise!("expected a name right after “--”");
                }
                previous = Some(String::from(name));
            } else if let Some(ref name) = previous {
                set!(name, chunk);
                previous = None;
            } else {
                arguments.orphans.push(chunk);
            }
        }
        if let Some(ref name) = previous {
            set!(name);
        }

        Ok(arguments)
    }
}

impl Arguments {
    /// Get the last value of an option (if present) converted to a specific
    /// type (if possible).
    pub fn get<T: FromStr>(&self, name: &str) -> Option<T> {
        self.options
            .get_ref::<Vec<String>>(name)
            .and_then(|strings| strings.last().and_then(|string| string.parse().ok()))
    }

    /// Get all values of an option (if present) converted to a specific type
    /// (if possible).
    pub fn get_all<T: FromStr>(&self, name: &str) -> Option<Vec<T>> {
        self.options
            .get_ref::<Vec<String>>(name)
            .and_then(|strings| {
                strings
                    .iter()
                    .map(|string| string.parse().ok())
                    .collect::<Option<Vec<_>>>()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Arguments;

    macro_rules! strings(($slices:expr) => ($slices.iter().map(|s| s.to_string())));

    #[test]
    fn program() {
        let arguments = vec!["--a", "--b"];
        match Arguments::parse(strings!(arguments)) {
            Ok(_) => unreachable!(),
            Err(_) => {}
        }
    }

    #[test]
    fn arrays() {
        let arguments = vec!["a", "--b", "1", "--b", "2"];
        let arguments = Arguments::parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get_all::<usize>("b").unwrap(), &[1, 2]);
    }

    #[test]
    fn booleans() {
        let arguments = vec!["a", "--b", "--no-c", "--d"];
        let arguments = Arguments::parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get::<bool>("b").unwrap(), true);
        assert_eq!(arguments.get::<bool>("c").unwrap(), false);
        assert_eq!(arguments.get::<bool>("d").unwrap(), true);
    }

    #[test]
    fn orphans() {
        let arguments = vec!["a", "b", "--c", "d", "e", "--f"];
        let Arguments { orphans, .. } = Arguments::parse(strings!(arguments)).unwrap();
        assert_eq!(&orphans, &["b", "e"]);
    }

    #[test]
    fn overrides() {
        let arguments = vec!["a", "--b", "1", "--b", "2"];
        let arguments = Arguments::parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get::<usize>("b").unwrap(), 2);
    }
}
