use clap::{Command, Parser, ValueEnum};

use crate::{input::SourceLineIterator, LineResult};

#[derive(Debug, Parser)]
#[command(version)]
#[allow(clippy::struct_excessive_bools)]
pub struct Opt {
    /// The input's minimum and maximum values
    #[arg(
        short,
        long,
        alias = "bounds",
        number_of_values = 2,
        allow_negative_numbers = true,
        value_names = ["MIN", "MAX"],
    )]
    range: Vec<f64>,

    /// Interpret arguments from the very first line of the input
    ///
    /// If this is passed, then the first line from standard input should match the following:
    ///
    /// ```plain
    /// braille [OPTIONS] [ARGUMENTS...]
    /// [VALUES...]
    /// ```
    ///
    /// Where `OPTIONS` and `ARGUMENTS` are space separated values as you would pass them on the
    /// command line, and `VALUES` are the values you want to graph.
    ///
    /// # Example
    ///
    /// `input.txt`
    ///
    /// ```plain
    /// braille -r -3 4 4
    /// -3
    /// -2
    /// -1
    /// 0
    /// 1
    /// 2
    /// 3
    /// 4
    /// ```
    ///
    /// ```console
    /// cat input.txt | braille --modeline
    /// ```
    ///
    /// ## Output
    ///
    /// ```plain
    /// ⠙⢿
    /// ⠀⢸⣷⣄
    /// ```
    #[arg(
        short,
        long,
        conflicts_with_all = [
            "range",
            "file",
            "kind",
            "columns",
            "braille",
            "size",
        ],
        verbatim_doc_comment
    )]
    pub modeline: bool,

    /// The kind of graph to print
    ///
    /// Kinds supported with their matching option parameters:
    ///
    /// | Kind    | Column    | Bar    |
    /// |---------|-----------|--------|
    /// | Braille | `braille` |        |
    /// | Block   | `columns` | `bars` |
    #[arg(short, long, value_enum, default_value_t, verbatim_doc_comment)]
    pub kind: GraphKind,

    /// Shortcut for --kind columns
    #[arg(short = 'C', conflicts_with = "kind")]
    pub columns: bool,

    /// Shortcut for --kind braille
    #[arg(short = 'B', conflicts_with = "kind")]
    pub braille: bool,

    /// Path to file to read from (defaults to standard input)
    #[arg(short, long, conflicts_with = "modeline")]
    pub file: Option<std::path::PathBuf>,

    /// Use the full height if none given
    ///
    /// By default, space is given for the prompt (either at the terminal or through a pager like
    /// `less`). Use this flag to instead take up the full height given. Passing a size overrides
    /// this flag. Does nothing if the graph is not vertical.
    #[arg(long)]
    pub use_full_default_height: bool,

    /// How wide or tall the graph can be (defaults to terminal size)
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub size: Option<u16>,

    #[arg(skip)]
    pub first_line: Option<FirstLine>,
}

#[derive(Debug)]
pub enum FirstLine {
    ModeLine,
    Value(String),
}

pub trait Configurable: From<Opt> {
    fn kind(&self) -> GraphKind;
    fn minimum(&self) -> f64;
    fn maximum(&self) -> f64;
    fn size(&self) -> u16;
}

#[derive(Debug)]
pub struct Config {
    kind: GraphKind,
    minimum: f64,
    maximum: f64,
    size: u16,
}

impl From<Opt> for Config {
    fn from(value: Opt) -> Self {
        if let [min, max] = value.range[0..2] {
            Self {
                kind: value.kind,
                minimum: min,
                maximum: max,
                size: value.size.unwrap(),
            }
        } else {
            unreachable!("The bounds should already have been calculated")
        }
    }
}

impl Configurable for Config {
    fn kind(&self) -> GraphKind {
        self.kind
    }

    fn minimum(&self) -> f64 {
        self.minimum
    }

    fn maximum(&self) -> f64 {
        self.maximum
    }

    fn size(&self) -> u16 {
        self.size
    }
}

/// Determine the terminal size from the terminal itself if possible, with fallbacks
fn get_terminal_size() -> anyhow::Result<(u16, u16)> {
    use terminal_size::{Height, Width};

    if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
        Ok((width, height))
    } else {
        use std::env::VarError;

        let parse_from_environment = |name, fallback| match std::env::var(name) {
            Ok(value) => Ok(value.parse()?),
            Err(VarError::NotPresent) => Ok(fallback),
            Err(err) => Err(anyhow::Error::from(err)),
        };

        let width = parse_from_environment("COLUMNS", 80)?;
        let height = parse_from_environment("LINES", 24)?;
        Ok((width, height))
    }
}

impl Opt {
    /// Parse options
    ///
    /// Call this instead of `Opt::parse()`, since it makes some adjustments not supported by
    /// [`clap`].
    pub fn try_new() -> anyhow::Result<Self> {
        let mut opt = Self::parse();

        // Parse the modeline if requested
        if opt.modeline {
            use clap::{CommandFactory, FromArgMatches};
            let mut cmd = Self::command_for_update().no_binary_name(true);
            cmd.build();

            let mut first_line = String::new();
            std::io::stdin().read_line(&mut first_line)?;

            if let Some(args) = Self::parse_modeline(&mut cmd, first_line.trim())? {
                let matches = cmd.get_matches_from(args);

                opt = Self::from_arg_matches(&matches)?;
                opt.first_line = Some(FirstLine::ModeLine);
            } else {
                opt.first_line = Some(FirstLine::Value(first_line));
            }
        } else {
            opt.first_line = None;
        }

        // Handle shortcuts
        if opt.columns {
            opt.kind = GraphKind::Columns;
        } else if opt.braille {
            opt.kind = GraphKind::BrailleLines;
        }

        // If the graph size isn't already set, try detecting it from the environment
        if opt.size.is_none() {
            let (width, height) = get_terminal_size()?;

            opt.size = Some(match opt.kind {
                GraphKind::Columns | GraphKind::BrailleLines => width,
                // Leave enough room for the shell prompt
                GraphKind::Bars | GraphKind::BrailleBars => {
                    if opt.use_full_default_height {
                        height
                    } else {
                        height - 1
                    }
                }
            });
        }

        Ok(opt)
    }

    /// Parse the first line as a modeline, or try parsing it as the first value
    fn parse_modeline<'a>(
        cmd: &mut Command,
        line: &'a str,
    ) -> Result<Option<Vec<&'a str>>, clap::Error> {
        if line.starts_with('#') {
            Ok(Some(vec![]))
        } else if line.starts_with("braille") {
            Ok(Some(
                line.trim_start_matches("braille")
                    .split_whitespace()
                    .take_while(|x| !x.starts_with('#'))
                    .collect(),
            ))
        } else if !line.is_empty() && line.parse::<f64>().is_err() {
            use clap::error::{ContextValue, ErrorKind};
            Err(cmd.error(
                ErrorKind::ValueValidation,
                ContextValue::String(format!(
                    r#"Invalid modeline: {line:?}

The first line should be the string "braille", followed by spaced separated options"#
                )),
            ))
        } else {
            Ok(None)
        }
    }

    /// If no bounds were given, look for them from the input and return the resulting iterator,
    /// otherwise simply return the resulting iterator.
    ///
    /// These are both wrapped inside an enum to allow for `impl Iterator<...>` types.
    pub fn get_iter(
        &mut self,
        input_lines: SourceLineIterator,
    ) -> anyhow::Result<ValueIter<impl Iterator<Item = LineResult> + 'static>> {
        let validate_bounds = |min: f64, max: f64| {
            if min > max {
                use clap::{error::ErrorKind, CommandFactory};
                let mut cmd = Self::command();
                anyhow::bail!(cmd.error(
                    ErrorKind::ValueValidation,
                    format!(
                        "min < max failed: {} < {}",
                        if min == f64::MAX {
                            "f64::MAX".to_string()
                        } else {
                            min.to_string()
                        },
                        if max == f64::MIN {
                            "f64::MIN".to_string()
                        } else {
                            max.to_string()
                        },
                    )
                ));
            }

            Ok(())
        };

        if let Some([min, max]) = self.range.get(0..2) {
            validate_bounds(*min, *max)?;

            match self.kind {
                GraphKind::Bars | GraphKind::BrailleBars => Ok(ValueIter::Bounded {
                    lines: input_lines.into_iter().collect(),
                }),
                GraphKind::Columns | GraphKind::BrailleLines => {
                    Ok(ValueIter::Boundless(input_lines.into_iter()))
                }
            }
        } else {
            let mut lines = vec![];
            let mut min = f64::MAX;
            let mut max = f64::MIN;

            for line in input_lines {
                if let Ok(Some(value)) = line {
                    min = min.min(value);
                    max = max.max(value);
                }
                lines.push(line);
            }

            validate_bounds(min, max)?;

            self.range = vec![min, max];

            Ok(ValueIter::Bounded { lines })
        }
    }
}

#[derive(Debug)]
pub enum ValueIter<BoundlessIter: Iterator<Item = LineResult> + 'static> {
    Boundless(BoundlessIter),
    Bounded { lines: Vec<LineResult> },
}

impl<BoundlessIter> IntoIterator for ValueIter<BoundlessIter>
where
    BoundlessIter: Iterator<Item = LineResult> + 'static,
{
    type Item = LineResult;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueIter::Boundless(lines) => Box::new(lines),
            ValueIter::Bounded { lines } => Box::new(lines.into_iter()),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum GraphKind {
    /// █▉▊▋▌▍▎▏ Column graph with block characters
    Columns,

    /// ⠙⣇ Column graph with braille characters
    ///
    /// ```plain
    /// ** *-
    /// -* *-
    /// -- *-
    /// -- **
    /// ```
    #[default]
    #[value(name = "braille")]
    BrailleLines,

    /// ⡶⠚ Bar graph with braille characters
    ///
    /// ```plain
    /// -- -*
    /// ** **
    /// ** --
    /// *- --
    /// ```
    BrailleBars,

    /// ▁▂▃▄▅▆▇█ Bar graph with block characters
    Bars,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn opt_sanity_check() {
        Opt::command().debug_assert();
    }
}
