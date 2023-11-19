use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

type LineResult = Result<Option<f64>, <f64 as FromStr>::Err>;

pub struct SourceLineIterator {
    iter: Box<dyn Iterator<Item = LineResult>>,
}

impl SourceLineIterator {
    pub fn try_from_path(first_value: Option<String>, path: Option<&Path>) -> anyhow::Result<Self> {
        let lines: Box<dyn Iterator<Item = LineResult>> = match path {
            None => Box::new(Self::get_lines(first_value)),
            Some(path) => {
                if path.as_os_str() == "-" {
                    Box::new(Self::get_lines(first_value))
                } else {
                    Box::new(Self::get_lines_from_file(path)?)
                }
            }
        };

        Ok(Self { iter: lines })
    }

    /// Parse the line as a float, and treat empty values or the lteral string "null" as
    /// missing
    fn parse_line(line: &str) -> LineResult {
        match line {
            l if l.is_empty() => Ok(None),
            "null" => Ok(None),
            _ => Some(line.parse()).transpose(),
        }
    }

    /// Parse input from stdin
    fn get_lines(first_value: Option<String>) -> impl Iterator<Item = LineResult> {
        let start = match first_value {
            Some(value) => vec![Ok(value)],
            None => vec![],
        };

        start
            .into_iter()
            .chain(std::io::stdin().lines())
            .map_while(Result::ok)
            .map(|x| Self::parse_line(&x))
    }

    /// Parse input from the given file path
    fn get_lines_from_file(path: &Path) -> anyhow::Result<impl Iterator<Item = LineResult>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(reader
            .lines()
            .map_while(Result::ok)
            .map(|x| Self::parse_line(&x)))
    }
}

impl IntoIterator for SourceLineIterator {
    type Item = LineResult;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_with_blank() {
        assert_eq!(Ok(None), SourceLineIterator::parse_line(""));
    }

    #[test]
    fn parse_line_with_null() {
        assert_eq!(Ok(None), SourceLineIterator::parse_line("null"));
    }

    #[test]
    fn parse_line_with_integer() {
        assert_eq!(Ok(Some(3.)), SourceLineIterator::parse_line("3"));
    }

    #[test]
    fn parse_line_with_rational() {
        assert_eq!(Ok(Some(2.5)), SourceLineIterator::parse_line("2.5"));
    }

    #[test]
    fn parse_line_with_non_number() {
        assert!(SourceLineIterator::parse_line("hello").is_err());
    }
}
