mod braille;
mod columns;

use crate::{LineResult, Opt};
pub use braille::BrailleLines;
pub use columns::Columns;

pub trait ColumnGraphable {
    type Item: std::str::FromStr;

    fn print_lines(opt: &Opt, lines: impl Iterator<Item = LineResult>) -> anyhow::Result<()>;
}
