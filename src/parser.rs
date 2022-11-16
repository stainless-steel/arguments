use {Arguments, Error, Options, Result};

/// A parser for command-line arguments.
pub struct Parser {
    _private: (),
}

impl Parser {
    /// Create a new parser.
    #[inline]
    pub fn new() -> Parser {
        Parser { _private: () }
    }

    /// Parse command-line arguments.
    pub fn parse<I: Iterator<Item = String>>(&self, mut stream: I) -> Result<Arguments> {
        let mut arguments = Arguments {
            program: match stream.next() {
                Some(ref program) if !program.starts_with("--") => String::from(&program[..]),
                _ => raise!("expected a name as the first argument"),
            },
            options: Options::new(),
            orphans: Vec::new(),
        };

        let mut previous: Option<String> = None;

        macro_rules! set(
            ($name:expr) => (
                if $name.starts_with("no-") {
                    if $name.len() == 3 {
                        raise!("expected a name right after “--no-”");
                    }
                    set!(&$name[3..], "false");
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
            if chunk.starts_with("--") {
                if let Some(ref name) = previous {
                    set!(name);
                }
                if chunk.len() == 2 {
                    raise!("expected a name right after “--”");
                }
                previous = Some(String::from(&chunk[2..]));
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

#[cfg(test)]
mod tests {
    use super::Parser;
    use Arguments;

    macro_rules! strings(
        ($slices:expr) => ($slices.iter().map(|s| s.to_string()));
    );

    #[test]
    fn program() {
        let arguments = vec!["--a", "--b"];
        match Parser::new().parse(strings!(arguments)) {
            Ok(_) => unreachable!(),
            Err(_) => {}
        }
    }

    #[test]
    fn arrays() {
        let arguments = vec!["a", "--b", "1", "--b", "2"];
        let arguments = Parser::new().parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get_all::<usize>("b").unwrap(), &[1, 2]);
    }

    #[test]
    fn booleans() {
        let arguments = vec!["a", "--b", "--no-c", "--d"];
        let arguments = Parser::new().parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get::<bool>("b").unwrap(), true);
        assert_eq!(arguments.get::<bool>("c").unwrap(), false);
        assert_eq!(arguments.get::<bool>("d").unwrap(), true);
    }

    #[test]
    fn orphans() {
        let arguments = vec!["a", "b", "--c", "d", "e", "--f"];
        let Arguments { orphans, .. } = Parser::new().parse(strings!(arguments)).unwrap();
        assert_eq!(&orphans, &["b", "e"]);
    }

    #[test]
    fn overrides() {
        let arguments = vec!["a", "--b", "1", "--b", "2"];
        let arguments = Parser::new().parse(strings!(arguments)).unwrap();
        assert_eq!(arguments.get::<usize>("b").unwrap(), 2);
    }
}
