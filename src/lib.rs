pub mod graph;
pub mod grid;
mod input;
mod opt;
pub mod util;

use std::io::prelude::*;
use std::io::LineWriter;

pub use graph::{
    blocks::{Bars as BlockBars, Columns as BlockColumns},
    braille::{Columns as BrailleColumns, Lines as BrailleLines},
    mini_blocks::{Columns as MiniBlockColumns, Lines as MiniBlockLines},
    sextants::{Columns as SextantColumns, Lines as SextantBars},
};
pub use graph::{BarGraphable, ColumnGraphable, Graphable};
use input::{
    Line as InputLine, LineResult, LineSinglable as InputLineSinglable, Lines as InputLines,
};
use opt::{Config, FirstLine};
pub use opt::{GraphKind, GraphStyle, Opt};

/// Main entry point for the program
pub fn run<W: Write>(opt: Opt, writer: LineWriter<W>) -> anyhow::Result<()> {
    if opt.grid.is_some() {
        grid::print_graph(opt, std::io::stdin().lock(), writer)
    } else {
        match (opt.kind(), opt.per) {
            (GraphKind::Bars, 1) => build_graph::<Option<f64>, BlockBars, W>(opt, writer),
            (GraphKind::MiniBars, 1) => build_graph::<Option<f64>, MiniBlockLines, W>(opt, writer),
            (GraphKind::MiniBars, 2) => {
                build_graph::<[Option<f64>; 2], MiniBlockLines, W>(opt, writer)
            }
            // (GraphKind::MiniBars, n) => build_graph::<Vec<Option<f64>>, MiniBlockLines, W>(opt, writer),
            (GraphKind::Columns, 1) => build_graph::<Option<f64>, BlockColumns, W>(opt, writer),
            (GraphKind::MiniColumns, 1) => {
                build_graph::<Option<f64>, MiniBlockColumns, W>(opt, writer)
            }
            (GraphKind::MiniColumns, 2) => {
                build_graph::<[Option<f64>; 2], MiniBlockColumns, W>(opt, writer)
            }
            (GraphKind::BrailleBars, 1) => build_graph::<Option<f64>, BrailleLines, W>(opt, writer),
            (GraphKind::BrailleBars, 2) => {
                build_graph::<[Option<f64>; 2], BrailleLines, W>(opt, writer)
            }
            (GraphKind::BrailleColumns, 1) => {
                build_graph::<Option<f64>, BrailleColumns, W>(opt, writer)
            }
            (GraphKind::BrailleColumns, 2) => {
                build_graph::<[Option<f64>; 2], BrailleColumns, W>(opt, writer)
            }
            (GraphKind::BrailleColumns, _) => {
                build_graph::<Vec<Option<f64>>, BrailleColumns, W>(opt, writer)
            }

            (GraphKind::SextantBars, 1) => build_graph::<Option<f64>, SextantBars, W>(opt, writer),
            (GraphKind::SextantBars, 2) => {
                build_graph::<[Option<f64>; 2], SextantBars, W>(opt, writer)
            }
            (GraphKind::SextantColumns, 1) => {
                build_graph::<Option<f64>, SextantColumns, W>(opt, writer)
            }
            (GraphKind::SextantColumns, 2) => {
                build_graph::<[Option<f64>; 2], SextantColumns, W>(opt, writer)
            }
            _ => todo!(),
        }
    }
}

fn build_graph<LineType, Graph, W>(mut opt: Opt, writer: LineWriter<W>) -> anyhow::Result<()>
where
    LineType: 'static,
    Graph: Graphable<LineType>,
    InputLine<LineType>: std::str::FromStr + for<'a> InputLineSinglable<'a>,
    <InputLine<LineType> as std::str::FromStr>::Err: std::error::Error + Send + Sync,
    W: Write,
{
    let first_value = match opt.first_line {
        Some(FirstLine::Value(ref value)) => Some(value.trim().to_string()),
        _ => None,
    };

    let lines = InputLines::<LineType>::try_from_path(first_value, opt.file.as_deref())?;

    let values = opt.get_iter(lines)?;
    let config = Config::from(opt);

    Graph::from(config).print_graph::<W>(values, writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_value_at_end_full_test() {
        let mut buffer = vec![];
        let writer = LineWriter::new(&mut buffer);
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/large_value_at_end.txt");

        let opt = Opt::try_new(["braille", "-f", path, "-c", "10"]).unwrap();

        run(opt, writer).unwrap();

        let output = String::from_utf8(buffer).unwrap();

        let expected = "\
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⡇
⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣤⣤⣤⣤⣤⣴⣶⣾⣿⣿⡇
";

        eprintln!("--- expected (start) ---\n{expected}--- expected (end) ---");
        eprintln!("--- actual (start) ---\n{output}--- actual (end) ---");

        assert_eq!(expected, output);
    }
}
