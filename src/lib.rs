#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub mod graph;
mod input;
mod opt;

use input::{InputLine, InputLineSinglable, InputLines};

pub use graph::{
    blocks::{Bars as BlockBars, Columns as BlockColumns},
    braille::{Columns as BrailleColumns, Lines as BrailleLines},
};
pub use graph::{BarGraphable, ColumnGraphable, Graphable};

use opt::{Config, FirstLine};
pub use opt::{GraphKind, Opt};

/// Main entry point for the program
pub fn run(opt: Opt) -> anyhow::Result<()> {
    match (opt.kind, opt.per) {
        (GraphKind::Bars, 1) => build_graph::<Option<f64>, BlockBars>(opt)?,
        (GraphKind::Columns, 1) => build_graph::<Option<f64>, BlockColumns>(opt)?,
        (GraphKind::BrailleBars, 1) => build_graph::<Option<f64>, BrailleLines>(opt)?,
        (GraphKind::BrailleBars, 2) => build_graph::<[Option<f64>; 2], BrailleLines>(opt)?,
        (GraphKind::BrailleColumns, 1) => build_graph::<Option<f64>, BrailleColumns>(opt)?,
        (GraphKind::BrailleColumns, 2) => build_graph::<[Option<f64>; 2], BrailleColumns>(opt)?,
        _ => todo!(),
    };

    Ok(())
}

fn build_graph<LineType: 'static, Graph>(mut opt: Opt) -> anyhow::Result<()>
where
    InputLine<LineType>: std::str::FromStr + InputLineSinglable,
    Graph: Graphable<LineType>,
{
    let first_value = match opt.first_line {
        Some(FirstLine::Value(ref value)) => Some(value.trim().to_string()),
        _ => None,
    };

    let lines = InputLines::<LineType>::try_from_path(first_value, opt.file.as_deref())?;

    let values = opt.get_iter(lines)?;
    let config = Config::from(opt);

    Graph::new(config).print_graph(values)?;

    Ok(())
}
