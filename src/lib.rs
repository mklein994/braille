pub mod graph;
mod input;
mod opt;

use std::io::prelude::*;
use std::io::LineWriter;
use std::str::FromStr;

use input::{
    Line as InputLine, LineResult, LineSinglable as InputLineSinglable, Lines as InputLines,
};

pub use graph::{
    blocks::{Bars as BlockBars, Columns as BlockColumns},
    braille::{Char as BrailleChar, Columns as BrailleColumns, Lines as BrailleLines},
    mini_blocks::{Columns as MiniBlockColumns, Lines as MiniBlockLines},
};
pub use graph::{BarGraphable, ColumnGraphable, Graphable};

use opt::{Config, FirstLine};
pub use opt::{GraphKind, GraphStyle, Opt};

/// Main entry point for the program
pub fn run<W: Write>(opt: Opt, writer: LineWriter<W>) -> anyhow::Result<()> {
    match (opt.kind(), opt.per) {
        (GraphKind::Bars, 1) => {
            if opt.pre_min().is_some_and(f64::is_sign_positive) {
                build_graph::<Option<f64>, BlockBars, W>(opt, writer)
            } else {
                build_graph::<Option<f64>, MiniBlockLines, W>(opt, writer)
            }
        }
        (GraphKind::Bars, 2) => build_graph::<[Option<f64>; 2], MiniBlockLines, W>(opt, writer),
        (GraphKind::Columns, 1) => {
            if opt.pre_min().is_some_and(f64::is_sign_positive) {
                build_graph::<Option<f64>, BlockColumns, W>(opt, writer)
            } else {
                build_graph::<Option<f64>, MiniBlockColumns, W>(opt, writer)
            }
        }
        (GraphKind::Columns, 2) => {
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
        _ => todo!(),
    }
}

fn build_graph<LineType: 'static, Graph, W: Write>(
    mut opt: Opt,
    writer: LineWriter<W>,
) -> anyhow::Result<()>
where
    InputLine<LineType>: FromStr + for<'a> InputLineSinglable<'a>,
    <InputLine<LineType> as FromStr>::Err: std::error::Error + Send + Sync,
    Graph: Graphable<LineType>,
{
    let first_value = match opt.first_line {
        Some(FirstLine::Value(ref value)) => Some(value.trim().to_string()),
        _ => None,
    };

    let lines = InputLines::<LineType>::try_from_path(first_value, opt.file.as_deref())?;

    let values = opt.get_iter(lines)?;
    let config = Config::from(opt);

    Graph::new(config).print_graph::<W>(values, writer)?;

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
