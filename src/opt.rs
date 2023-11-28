use std::str::FromStr;

use clap::{builder::BoolishValueParser, Command, Parser, ValueEnum};

use crate::input::{InputLine, InputLineResult as LineResult, InputLineSinglable, InputLines};

#[derive(Debug, Copy, Clone, Default)]
struct GraphRangeBound(Option<f64>);

impl std::fmt::Display for GraphRangeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.map(|x| x.to_string()).unwrap_or_default().fmt(f)
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct GraphRange {
    min: GraphRangeBound,
    max: GraphRangeBound,
}

impl GraphRange {
    fn new(min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            min: GraphRangeBound(min),
            max: GraphRangeBound(max),
        }
    }

    fn min(&self) -> Option<f64> {
        self.min.0
    }

    fn max(&self) -> Option<f64> {
        self.max.0
    }
}

impl std::fmt::Display for GraphRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.min, self.max)
    }
}

impl std::str::FromStr for GraphRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == ":" {
            return Ok(Self::default());
        }

        if !s.contains(':') {
            anyhow::bail!("Invalid range syntax");
        }

        let maybe_parse = |value: &str| -> Result<Option<f64>, <f64 as std::str::FromStr>::Err> {
            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(value.parse()?))
            }
        };

        let (min, max) = {
            let (min, max) = s.split_once(':').expect("Invalid match range syntax");
            (maybe_parse(min)?, maybe_parse(max)?)
        };

        Ok(Self::new(min, max))
    }
}

#[derive(Debug, Default, ValueEnum, Clone, Copy)]
pub enum GraphStyle {
    /// Fill space between series when the first is less or equal to the second value,
    /// hollow otherwise
    #[value(alias = "a")]
    Auto,

    /// Never fill between series (show each independently as a line)
    #[value(alias = "l")]
    Line,

    /// Always fill the space between multiple series
    #[default]
    #[value(alias = "f")]
    Filled,
}

#[derive(Debug, Parser)]
#[command(version)]
#[allow(clippy::struct_excessive_bools)]
pub struct Opt {
    /// The input's minimum and maximum values
    ///
    /// If provided, at least one of `MIN` or `MAX` must be given.
    ///
    /// # Example
    ///
    /// ```plain
    /// --range -3:4  # Use bounds given
    /// --range -3:   # Automatically determine maximum
    /// --range :4    # Automatically determine minimum
    /// ```
    #[arg(
        short,
        long,
        alias = "bounds",
        verbatim_doc_comment,
        allow_hyphen_values = true,
        default_value_t,
        hide_default_value = true,
        value_name = "[MIN]:[MAX]"
    )]
    range: GraphRange,

    /// Number of values per line of input
    ///
    /// When the graph kind supports it, each value represents a separate series.
    #[arg(short, long, default_value_t = 1, value_parser(clap::value_parser!(u8).range(1..=2)))]
    pub per: u8,

    /// How the space between multiple series should be handled
    #[arg(short, long, value_enum, default_value_t)]
    pub style: GraphStyle,

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
            "bars",
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
    /// | Kind    | Bar (horizontal) | Column (vertical)      |
    /// |---------|------------------|------------------------|
    /// | Braille | `braille` (-b)   | `braille-columns` (-c) |
    /// | Block   | `bars` (-B)      | `columns` (-C)         |
    #[arg(short, long, value_enum, default_value_t, verbatim_doc_comment)]
    pub kind: GraphKind,

    /// Shortcut for --kind bars
    #[arg(short = 'B', conflicts_with = "kind")]
    pub bars: bool,

    /// Shortcut for --kind columns
    #[arg(short = 'C', conflicts_with = "kind")]
    pub columns: bool,

    /// Shortcut for --kind braille
    #[arg(short = 'b', conflicts_with = "kind")]
    pub braille: bool,

    /// Shortcut for --kind braille-columns
    #[arg(short = 'c', conflicts_with = "kind")]
    pub braille_columns: bool,

    /// Path to file to read from (defaults to standard input)
    #[arg(short, long, conflicts_with = "modeline")]
    pub file: Option<std::path::PathBuf>,

    /// Use the full height if none given
    ///
    /// By default, space is given for the prompt (either at the terminal or through a pager like
    /// `less`). Use this flag to instead take up the full height given. Passing a size overrides
    /// this flag. Does nothing if the graph is not vertical.
    #[arg(long, env = "BRAILLE_USE_FULL_DEFAULT_HEIGHT", value_parser = BoolishValueParser::new())]
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
    fn style(&self) -> GraphStyle;
    fn minimum(&self) -> f64;
    fn maximum(&self) -> f64;
    fn size(&self) -> u16;
}

#[derive(Debug)]
pub struct Config {
    kind: GraphKind,
    style: GraphStyle,
    minimum: f64,
    maximum: f64,
    size: u16,
}

impl From<Opt> for Config {
    fn from(value: Opt) -> Self {
        if let (Some(min), Some(max)) = (value.range.min(), value.range.max()) {
            Self {
                kind: value.kind,
                style: value.style,
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

    fn style(&self) -> GraphStyle {
        self.style
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
        if opt.bars {
            opt.kind = GraphKind::Bars;
        } else if opt.braille {
            opt.kind = GraphKind::BrailleBars;
        } else if opt.braille_columns {
            opt.kind = GraphKind::BrailleColumns;
        } else if opt.columns {
            opt.kind = GraphKind::Columns;
        }

        match (opt.kind, opt.per) {
            (GraphKind::Columns | GraphKind::Bars, x) if x > 1 => {
                anyhow::bail!("Multiple values per line not supported for this graph kind");
            }
            _ => {}
        }

        // If the graph size isn't already set, try detecting it from the environment
        if opt.size.is_none() {
            let (width, height) = get_terminal_size()?;

            opt.size = Some(match opt.kind {
                GraphKind::Bars | GraphKind::BrailleBars => width,
                // Leave enough room for the shell prompt
                GraphKind::Columns | GraphKind::BrailleColumns => {
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
    pub fn get_iter<T>(&mut self, input_lines: InputLines<T>) -> anyhow::Result<ValueIter<T>>
    where
        InputLine<T>: FromStr + InputLineSinglable,
    {
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

        if let (Some(min), Some(max)) = (self.range.min(), self.range.max()) {
            validate_bounds(min, max)?;

            match self.kind {
                GraphKind::Bars | GraphKind::BrailleBars => {
                    Ok(ValueIter::Boundless(input_lines.into_iter()))
                }
                GraphKind::Columns | GraphKind::BrailleColumns => Ok(ValueIter::Bounded {
                    lines: input_lines.into_iter().collect(),
                }),
            }
        } else {
            let mut lines = vec![];
            let has_min = self.range.min().is_some();
            let has_max = self.range.max().is_some();
            assert!(!has_min || !has_max);
            let mut min = self.range.min().unwrap_or(f64::MAX);
            let mut max = self.range.max().unwrap_or(f64::MIN);

            for line in input_lines {
                for value in line.iter().flat_map(|line| line.as_single_iter().flatten()) {
                    if !has_min {
                        min = min.min(value);
                    }

                    if !has_max {
                        max = max.max(value);
                    }
                }

                lines.push(line);
            }

            validate_bounds(min, max)?;

            self.range = GraphRange::new(Some(min), Some(max));

            Ok(ValueIter::Bounded { lines })
        }
    }
}

pub enum ValueIter<T>
where
    InputLine<T>: FromStr,
{
    Boundless(InputLines<T>),
    Bounded { lines: Vec<LineResult<T>> },
}

impl<T: 'static> IntoIterator for ValueIter<T>
where
    InputLine<T>: FromStr,
{
    type Item = LineResult<T>;

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
    /// █▉▊▋▌▍▎▏ Bar graph with block characters
    Bars,

    /// ▁▂▃▄▅▆▇█ Column graph with block characters
    Columns,

    /// ⠙⣇ Bar graph with braille characters
    ///
    /// ```plain
    /// ** *-
    /// -* *-
    /// -- *-
    /// -- **
    /// ```
    #[default]
    #[value(name = "braille")]
    BrailleBars,

    /// ⡶⠚ Column graph with braille characters
    ///
    /// ```plain
    /// -- -*
    /// ** **
    /// ** --
    /// *- --
    /// ```
    BrailleColumns,
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
