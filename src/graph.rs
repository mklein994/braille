pub mod blocks;
pub mod braille;

use std::str::FromStr;

use crate::input::InputLine;
use crate::opt::{Config, Configurable, ValueIter};

pub trait Graphable<T, Conf: Configurable = Config>
where
    InputLine<T>: FromStr,
{
    fn new(config: Conf) -> Self;

    fn config(&self) -> &Conf;

    fn minimum(&self) -> f64 {
        self.config().minimum()
    }

    fn maximum(&self) -> f64 {
        self.config().maximum()
    }

    fn print_graph(&self, lines: ValueIter<T>) -> anyhow::Result<()>
    where
        InputLine<T>: FromStr;
}

pub trait BarGraphable<T>: Graphable<T>
where
    InputLine<T>: FromStr,
{
    fn width(&self) -> u16 {
        self.config().size()
    }
}

pub trait ColumnGraphable<T>: Graphable<T>
where
    InputLine<T>: FromStr,
{
    fn height(&self) -> u16 {
        self.config().size()
    }
}
