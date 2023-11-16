use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Opt {
    /// The kind of graph to print
    #[arg(short, long, value_enum, default_value_t)]
    pub kind: GraphKind,

    /// Shortcut for --kind columns
    #[arg(short = 'C', conflicts_with = "kind")]
    pub columns: bool,

    /// Shortcut for --kind braille
    #[arg(short = 'B', conflicts_with = "kind")]
    pub braille: bool,

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
    pub size: Option<u16>,
}

pub trait Configurable: for<'a> From<&'a Opt> {
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

impl From<&Opt> for Config {
    fn from(value: &Opt) -> Self {
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

impl Opt {
    /// Parse options
    ///
    /// Call this instead of `Opt::parse()`, since it makes some adjustments not supported by
    /// [`clap`].
    #[must_use]
    pub fn new() -> Self {
        let mut opt = Self::parse();

        if opt.columns {
            opt.kind = GraphKind::Columns;
        } else if opt.braille {
            opt.kind = GraphKind::BrailleLines;
        }

        // The "minimum" value will be parsed as the size if a maximum is not provided
        if let (Some(size), None) = (opt.minimum, opt.maximum) {
            opt.size = Some(size as u16);
        } else if opt.size.is_none() {
            use terminal_size::{Height, Width};

            let (width, height) = terminal_size::terminal_size()
                .map_or((80, 24), |(Width(width), Height(height))| (width, height));

            opt.size = Some(match opt.kind {
                GraphKind::Columns | GraphKind::BrailleLines => width,
                GraphKind::BrailleBars => height,
            });
        }

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
    #[value(name = "braille")]
    BrailleLines,

    BrailleBars,
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
