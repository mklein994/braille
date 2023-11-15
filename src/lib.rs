pub mod graph;
use std::io::prelude::*;
mod opt;

use std::fs::File;

pub use graph::{Braille, ColumnGraphable, Columns};
pub use opt::{GraphKind, Opt};

pub fn run(opt: &mut Opt) -> anyhow::Result<()> {
    match &opt.file {
        Some(path) => {
            let input_lines = get_lines_from_file(path)?;

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
                print_lines(opt, lines.into_iter())?;
            } else {
                print_lines(opt, input_lines)?;
            };
        }
        None => {
            let input_lines = get_lines();

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
                print_lines(opt, lines.into_iter())?;
            } else {
                print_lines(opt, input_lines)?;
            };
        }
    };

    Ok(())
}

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
pub fn get_lines() -> impl Iterator<Item = Result<Option<f64>, <f64 as std::str::FromStr>::Err>> {
    std::io::stdin().lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    })
}

/// Parse standard input as a list of numbers (blank lines are treated as missing values)
pub fn get_lines_from_file(
    path: &std::path::Path,
) -> anyhow::Result<impl Iterator<Item = Result<Option<f64>, <f64 as std::str::FromStr>::Err>>> {
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
