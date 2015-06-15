use {Arguments, Options};

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
    pub fn parse<I: Iterator<Item=String>>(&self, mut stream: I) -> Result<Arguments, String> {
        let mut arguments = Arguments {
            program: match stream.next() {
                Some(program) => String::from(program),
                _ => raise!("expected at least the name of the executed program"),
            },
            options: Options::new(),
            orphans: Vec::new(),
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

#[cfg(test)]
mod tests {
    use Arguments;
    use super::Parser;

    macro_rules! strings(
        ($($str:expr),*) => (
            [$($str),*].iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
    );

    #[test]
    fn orphans() {
        let args = strings!["a", "b", "--c", "d", "e", "--f"];
        let args = args.iter().map(|a| a.to_string());
        let Arguments { orphans, .. } = Parser::new().parse(args).unwrap();
        assert_eq!(orphans, strings!["b", "e"]);
    }
}
