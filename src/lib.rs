pub mod graph;
mod opt;

pub use graph::{Braille, ColumnGraphable, Columns};
pub use opt::{GraphKind, Opt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// The type used as the iterator item while parsing lines
pub type LineResult = Result<Option<f64>, <f64 as std::str::FromStr>::Err>;

/// Main entry point for the program
pub fn run(opt: &mut Opt) -> anyhow::Result<()> {
    let lines: Box<dyn Iterator<Item = LineResult>> = match &opt.file {
        None => Box::new(get_lines()),
        Some(path) => {
            if path.as_os_str() == "-" {
                Box::new(get_lines())
            } else {
                Box::new(get_lines_from_file(path)?)
            }
        }
    };

    let line_iter = maybe_detect_bounds(opt, lines);

    print_lines(opt, line_iter)?;

    Ok(())
}

enum LineIter<BoundlessIter: Iterator<Item = LineResult>, BoundedIter: Iterator<Item = LineResult>>
{
    Boundless(BoundlessIter),
    Bounded(BoundedIter),
}

impl<BoundlessIter, BoundedIter> Iterator for LineIter<BoundlessIter, BoundedIter>
where
    BoundlessIter: Iterator<Item = LineResult>,
    BoundedIter: Iterator<Item = LineResult>,
{
    type Item = LineResult;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Boundless(iter) => iter.next(),
            Self::Bounded(iter) => iter.next(),
        }
    }
}

/// If no bounds were given, look for them from the input and return the resulting iterator
fn maybe_detect_bounds(
    opt: &mut Opt,
    input_lines: impl Iterator<Item = LineResult>,
) -> LineIter<impl Iterator<Item = LineResult>, impl Iterator<Item = LineResult>> {
    if opt.minimum.and(opt.maximum).is_none() {
        let mut lines = vec![];
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for line in input_lines {
            if let Ok(Some(value)) = line {
                min = min.min(value);
                max = max.max(value);
            }
            lines.push(line);
        }

        opt.minimum = Some(min);
        opt.maximum = Some(max);

        LineIter::Bounded(lines.into_iter())
    } else {
        LineIter::Boundless(input_lines)
    }
}

/// Print the graph using the options and input lines
fn print_lines(opt: &Opt, lines: impl Iterator<Item = LineResult>) -> anyhow::Result<()> {
    match opt.kind {
        GraphKind::Columns => Columns::print_lines(opt, lines),
        GraphKind::Braille => Braille::print_lines(opt, lines),
    }
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
        .map(|x| parse_line(&x))
}

/// Parse input from the given file path
fn get_lines_from_file(path: &Path) -> anyhow::Result<impl Iterator<Item = LineResult>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map_while(Result::ok).map(|x| parse_line(&x)))
}
