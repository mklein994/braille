mod braille;
mod colblock;

use crate::Opt;
pub use braille::Braille;
pub use colblock::ColBlock;

pub trait ColumnGraphable {
    type Item;
    type Error: std::error::Error;

    fn print_lines<I: Iterator<Item = anyhow::Result<Self::Item, Self::Error>>>(
        opt: &Opt,
        input_lines: I,
    ) -> anyhow::Result<()>;
}
