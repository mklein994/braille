#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub mod graph;
mod opt;

pub use graph::{BarGraphable, ColumnGraphable, Graphable};
pub use graph::{Bars, Columns, Lines};

use opt::Config;
use opt::Configurable;
use opt::LineIter;
pub use opt::{GraphKind, Opt};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// The type used as the iterator item while parsing lines
pub type LineResult = Result<Option<f64>, <f64 as std::str::FromStr>::Err>;

/// Main entry point for the program
pub fn run(mut opt: Opt) -> anyhow::Result<()> {
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

    // TODO: figure out how to couple these together
    // It shouldn't be possible to build the config before creating the iterator
    let line_iter = opt.get_iter(lines)?;
    let config = Config::from(opt);

    print_lines(config, line_iter)?;

    Ok(())
}

/// Print the graph using the options and input lines
fn print_lines(
    config: Config,
    iter: LineIter<impl Iterator<Item = LineResult> + 'static>,
) -> anyhow::Result<()> {
    match (config.kind(), iter) {
        (GraphKind::Columns, iter) => Columns::new(config).print_lines(iter.into_iter()),
        (GraphKind::BrailleLines, iter) => Lines::new(config).print_lines(iter.into_iter()),
        (GraphKind::Bars, LineIter::Bounded { lines }) => Bars::new(config).print_bars(lines),
        (GraphKind::BrailleBars, LineIter::Bounded { .. }) => todo!(),
        (kind, _) => panic!("Unknown graph/iter combo: {kind:?}"),
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
