pub mod graph;
use std::io::prelude::*;
mod opt;

use std::fs::File;

pub use graph::{Braille, ColumnGraphable, Columns};
pub use opt::{GraphKind, Opt};

pub fn print_lines(
    opt: &Opt,
    lines: impl Iterator<Item = Result<Option<f64>, std::num::ParseFloatError>>,
) -> anyhow::Result<()> {
    match opt.kind {
        GraphKind::Columns => Columns::print_lines(opt, lines),
        GraphKind::Braille => Braille::print_lines(opt, lines),
    }
}

/// Parse standard input as a list of numbers (blank lines are treated as missing values)
pub fn get_lines<T: std::str::FromStr>(
) -> impl Iterator<Item = Result<Option<T>, <T as std::str::FromStr>::Err>> {
    std::io::stdin().lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    })
}

/// Parse standard input as a list of numbers (blank lines are treated as missing values)
pub fn get_lines_from_file<T: std::str::FromStr>(
    path: &std::path::Path,
) -> anyhow::Result<impl Iterator<Item = Result<Option<T>, <T as std::str::FromStr>::Err>>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    Ok(reader.lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    }))
}
