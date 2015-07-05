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
    pub fn parse<I: Iterator<Item=String>>(&self, mut stream: I) -> Result<Arguments> {
        let mut arguments = Arguments {
            program: match stream.next() {
                Some(program) => String::from(program),
                _ => raise!("expected a name as the first argument"),
            },
            options: Options::new(),
            orphans: Vec::new(),
        };

        let mut previous: Option<String> = None;

        macro_rules! set_boolean_if_any(
            () => (
                if let Some(ref name) = previous {
                    if name.starts_with("no-") {
                        if name.len() == 3 {
                            raise!("expected a name right after “--no-”");
                        }
                        arguments.options.set(&name[3..], "false".to_string());
                    } else {
                        arguments.options.set(name, "true".to_string());
                    }
                }
            );
        );

        for chunk in stream {
            if chunk.starts_with("--") {
                set_boolean_if_any!();
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
        set_boolean_if_any!();

        Ok(arguments)
    }
}

#[cfg(test)]
mod tests {
    use Arguments;
    use super::Parser;

    macro_rules! strings(
        ($slices:expr) => ($slices.iter().map(|s| s.to_string()));
    );

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
}
