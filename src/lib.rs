#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub mod graph;
mod input;
mod opt;

pub use graph::{BarGraphable, ColumnGraphable, Graphable};
pub use graph::{Bars, Columns, Lines};

use input::SourceLineIterator;
use opt::Config;
use opt::Configurable;
use opt::ValueIter;
pub use opt::{GraphKind, Opt};

/// The type used as the iterator item while parsing lines
pub type LineResult = Result<Option<f64>, <f64 as std::str::FromStr>::Err>;

/// Main entry point for the program
pub fn run(mut opt: Opt) -> anyhow::Result<()> {
    let lines = SourceLineIterator::try_from_path(opt.file.as_deref())?;

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
        (GraphKind::Columns, values) => Columns::new(config).print_lines(values.into_iter()),
        (GraphKind::BrailleLines, values) => Lines::new(config).print_lines(values.into_iter()),
        (GraphKind::Bars, ValueIter::Bounded { lines }) => Bars::new(config).print_bars(lines),
        (GraphKind::BrailleBars, ValueIter::Bounded { .. }) => todo!(),
        (kind, _) => panic!("Unknown graph/iter combo: {kind:?}"),
    }
}
