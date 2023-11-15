pub mod graph;
mod opt;

pub use graph::{Braille, ColumnGraphable, Columns};
pub use opt::{GraphKind, Opt};

/// Parse standard input as a list of numbers (blank lines are treated as missing values)
pub fn get_lines() -> impl Iterator<Item = Result<Option<f64>, std::num::ParseFloatError>> {
    std::io::stdin().lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    })
}

pub fn print_lines(
    opt: &Opt,
    input_lines: impl Iterator<Item = Result<Option<f64>, std::num::ParseFloatError>>,
) -> anyhow::Result<()> {
    match opt.kind {
        GraphKind::Columns => Columns::print_lines(opt, input_lines),
        GraphKind::Braille => Braille::print_lines(opt, input_lines),
    }
}
