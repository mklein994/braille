mod braille;
mod columns;

use crate::Opt;
pub use braille::Braille;
pub use columns::Columns;

pub trait ColumnGraphable {
    type Item: std::str::FromStr;

    fn print_lines(
        opt: &Opt,
        lines: impl Iterator<Item = Result<Option<Self::Item>, <Self::Item as std::str::FromStr>::Err>>,
    ) -> anyhow::Result<()>;
}
