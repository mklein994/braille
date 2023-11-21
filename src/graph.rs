pub mod blocks;
pub mod braille;

use crate::opt::{Config, Configurable};
use crate::LineResult;

pub trait Graphable<Conf: Configurable = Config> {
    fn new(config: Conf) -> Self;

    fn config(&self) -> &Conf;

    fn minimum(&self) -> f64 {
        self.config().minimum()
    }

    fn maximum(&self) -> f64 {
        self.config().maximum()
    }
}

pub trait BarGraphable: Graphable {
    fn width(&self) -> u16 {
        self.config().size()
    }

    fn print_bars(&self, lines: impl Iterator<Item = LineResult>) -> anyhow::Result<()>;
}

pub trait ColumnGraphable: Graphable {
    fn height(&self) -> u16 {
        self.config().size()
    }

    fn print_columns(&self, lines: Vec<LineResult>) -> anyhow::Result<()>;
}
