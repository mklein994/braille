#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub mod graph;
mod input;
mod opt;

pub use graph::{BarGraphable, ColumnGraphable, Graphable};
pub use graph::{Bars, Columns, Lines};

use input::SourceLineIterator;
use opt::Config;
use opt::Configurable;
use opt::LineIter;
pub use opt::{GraphKind, Opt};

/// The type used as the iterator item while parsing lines
pub type LineResult = Result<Option<f64>, <f64 as std::str::FromStr>::Err>;

/// Main entry point for the program
pub fn run(mut opt: Opt) -> anyhow::Result<()> {
    let lines = SourceLineIterator::try_from_path(opt.file.as_deref())?;

    let line_iter = opt.get_iter(lines)?;
    let config = Config::from(opt);

    print_lines(config, line_iter)?;

    Ok(())
}

/// Print the graph using the options and input lines
fn print_lines(
    config: Config,
    iter: LineIter<impl Iterator<Item = LineResult> + 'static>,
) -> anyhow::Result<()> {
    match (config.kind(), iter) {
        (GraphKind::Columns, iter) => Columns::new(config).print_lines(iter.into_iter()),
        (GraphKind::BrailleLines, iter) => Lines::new(config).print_lines(iter.into_iter()),
        (GraphKind::Bars, LineIter::Bounded { lines }) => Bars::new(config).print_bars(lines),
        (GraphKind::BrailleBars, LineIter::Bounded { .. }) => todo!(),
        (kind, _) => panic!("Unknown graph/iter combo: {kind:?}"),
    }
}
