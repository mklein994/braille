pub mod blocks;
pub mod braille;
pub mod mini_blocks;

use std::io::{LineWriter, Write};
use std::str::FromStr;

use crate::opt::{Config, Configurable, GraphStyle, ValueIter};
use crate::InputLine;

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

    fn style(&self) -> GraphStyle {
        self.config().style()
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<T>,
        writer: LineWriter<W>,
    ) -> anyhow::Result<()>
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
