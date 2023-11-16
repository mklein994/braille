mod bars;
mod braille;
mod columns;

use crate::opt::Configurable;
use crate::LineResult;
pub use bars::Bars;
pub use braille::Lines;
pub use columns::Columns;

pub trait Graphable {
    type Config: Configurable;
    type Item: std::str::FromStr;

    fn new(config: Self::Config) -> Self;

    fn config(&self) -> &Self::Config;

    fn minimum(&self) -> f64 {
        self.config().minimum()
    }

    fn maximum(&self) -> f64 {
        self.config().maximum()
    }
}

pub trait ColumnGraphable: Graphable {
    fn width(&self) -> u16 {
        self.config().size()
    }

    fn print_lines(&self, lines: impl Iterator<Item = LineResult>) -> anyhow::Result<()>;
}

pub trait BarGraphable: Graphable {
    fn height(&self) -> u16 {
        self.config().size()
    }

    fn print_bars(&self, lines: Vec<LineResult>) -> anyhow::Result<()>;
}
