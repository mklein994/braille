use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

type LineResult = Result<Option<f64>, <f64 as FromStr>::Err>;

pub struct SourceLineIterator {
    iter: Box<dyn Iterator<Item = LineResult>>,
}

impl SourceLineIterator {
    pub fn try_from_path(path: Option<&Path>) -> anyhow::Result<Self> {
        let lines: Box<dyn Iterator<Item = LineResult>> = match path {
            None => Box::new(Self::get_lines()),
            Some(path) => {
                if path.as_os_str() == "-" {
                    Box::new(Self::get_lines())
                } else {
                    Box::new(Self::get_lines_from_file(path)?)
                }
            }
        };

        Ok(Self { iter: lines })
    }

    /// Parse the line as a float, and treat empty values as missing
    fn parse_line(line: &str) -> LineResult {
        if line.is_empty() {
            Ok(None)
        } else {
            Some(line.parse()).transpose()
        }
    }

    /// Parse input from stdin
    fn get_lines() -> impl Iterator<Item = LineResult> {
        std::io::stdin()
            .lines()
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
