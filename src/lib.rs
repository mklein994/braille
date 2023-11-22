#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub mod graph;
mod input;
mod opt;

pub use graph::{
    blocks::{Bars as BlockBars, Columns as BlockColumns},
    braille::{Columns as BrailleColumns, Lines as BrailleLines},
};
pub use graph::{BarGraphable, ColumnGraphable, Graphable};

use input::SourceLineIterator;
use opt::Configurable;
use opt::ValueIter;
use opt::{Config, FirstLine};
pub use opt::{GraphKind, Opt};

pub use input::Line as LineResult;

/// Main entry point for the program
pub fn run(mut opt: Opt) -> anyhow::Result<()> {
    let first_value = match &opt.first_line {
        Some(FirstLine::Value(value)) => Some(value.trim().to_string()),
        _ => None,
    };
    let lines = SourceLineIterator::try_from_path(first_value, opt.file.as_deref())?;

    let values = opt.get_iter(lines)?;
    let config = Config::from(opt);

    print_graph(config, values)?;

    Ok(())
}

/// Print the graph using the options and input lines
fn print_graph(
    config: Config,
    values: ValueIter<impl Iterator<Item = LineResult> + 'static>,
) -> anyhow::Result<()> {
    match (config.kind(), values) {
        (GraphKind::Bars, values) => BlockBars::new(config).print_bars(values.into_iter()),
        (GraphKind::BrailleBars, values) => {
            BrailleLines::new(config).print_bars(values.into_iter())
        }
        (GraphKind::Columns, ValueIter::Bounded { lines }) => {
            BlockColumns::new(config).print_columns(lines)
        }
        (GraphKind::BrailleColumns, ValueIter::Bounded { lines }) => {
            BrailleColumns::new(config).print_columns(lines)
        }
        (kind, _) => panic!("Unknown graph/iter combo: {kind:?}"),
    }
}
