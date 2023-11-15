use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Opt {
    /// The kind of graph to print
    #[arg(short, long, value_enum, default_value_t)]
    pub kind: GraphKind,

    /// Path to file to read from (defaults to standard input)
    #[arg(short, long)]
    pub file: Option<std::path::PathBuf>,

    /// The input's minimum value
    #[arg(allow_negative_numbers = true)]
    pub minimum: Option<f64>,

    /// The input's maximum value
    #[arg(allow_negative_numbers = true, requires = "minimum")]
    pub maximum: Option<f64>,

    /// How wide the graph can be (defaults to terminal width)
    #[arg(default_value_t, hide_default_value = true)]
    width: Width,
}

impl Opt {
    /// Parse options
    ///
    /// Call this instead of `Opt::parse()`, since it makes some adjustments not supported by
    /// [`clap`].
    #[must_use]
    pub fn new() -> Self {
        let mut opt = Self::parse();
        if let (Some(min), None) = (opt.minimum, opt.maximum) {
            opt.width = Width(min as u16);
        };
        opt
    }
}

impl Default for Opt {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum GraphKind {
    Columns,
    #[default]
    Braille,
}

impl Opt {
    /// The width of the graph
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width.0
    }
}

#[derive(Debug, Clone, Copy)]
struct Width(u16);

impl Default for Width {
    fn default() -> Self {
        match terminal_size::terminal_size() {
            Some((terminal_size::Width(width), _)) => Self(width),
            None => Self(80),
        }
    }
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl clap::builder::ValueParserFactory for Width {
    type Parser = WidthValueParser;
    fn value_parser() -> Self::Parser {
        WidthValueParser
    }
}

#[derive(Clone, Debug)]
struct WidthValueParser;
impl clap::builder::TypedValueParser for WidthValueParser {
    type Value = Width;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u16).range(1..);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Width(val))
    }
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
