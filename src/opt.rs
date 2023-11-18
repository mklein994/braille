use clap::{Command, Parser, ValueEnum};

use crate::{input::SourceLineIterator, LineResult};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Opt {
    /// Interpret arguments from the very first line of the input
    ///
    /// If this is passed, then the first line from standard input should match the following:
    ///
    /// ```plain
    /// braille: [OPTIONS] [ARGUMENTS...]
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
    /// braille: -3 4 4
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
        exclusive = true,
        conflicts_with = "file",
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

    /// The input's minimum value
    #[arg(allow_negative_numbers = true)]
    pub minimum: Option<f64>,

    /// The input's maximum value
    #[arg(allow_negative_numbers = true, requires = "minimum")]
    pub maximum: Option<f64>,

    /// How wide or tall the graph can be (defaults to terminal size)
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
        Self {
            kind: value.kind,
            minimum: value.minimum.unwrap(),
            maximum: value.maximum.unwrap(),
            size: value.size.unwrap(),
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

        let parse_from_environment = |name, default| match std::env::var(name) {
            Ok(value) => Ok(value.parse()?),
            Err(VarError::NotPresent) => Ok(default),
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

        if opt.modeline {
            use clap::{CommandFactory, FromArgMatches};
            let mut cmd = Self::command_for_update()
                .no_binary_name(true)
                .mut_arg("modeline", |arg| arg.exclusive(false))
                .mut_arg("file", |arg| arg.conflicts_with("modeline"));
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

        if opt.columns {
            opt.kind = GraphKind::Columns;
        } else if opt.braille {
            opt.kind = GraphKind::BrailleLines;
        }

        // The "minimum" value will be parsed as the size if a maximum is not provided
        if let (Some(size), None) = (opt.minimum, opt.maximum) {
            opt.size = Some(size as u16);
        } else if opt.size.is_none() {
            let (width, height) = get_terminal_size()?;

            opt.size = Some(match opt.kind {
                GraphKind::Columns | GraphKind::BrailleLines => width,
                // Leave enough room for the shell prompt
                GraphKind::Bars | GraphKind::BrailleBars => height - 1,
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
        let validate_bounds = |min, max| {
            if min > max {
                use clap::{error::ErrorKind, CommandFactory};
                let mut cmd = Self::command();
                anyhow::bail!(cmd.error(
                    ErrorKind::ValueValidation,
                    format!("min < max failed: {min} < {max}")
                ));
            }

            Ok(())
        };

        if let (Some(min), Some(max)) = (self.minimum, self.maximum) {
            validate_bounds(min, max)?;

            if matches!(self.kind, GraphKind::Bars | GraphKind::BrailleBars) {
                Ok(ValueIter::Bounded {
                    lines: input_lines.into_iter().collect(),
                })
            } else {
                Ok(ValueIter::Boundless(input_lines.into_iter()))
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

            self.minimum = Some(min);
            self.maximum = Some(max);

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
