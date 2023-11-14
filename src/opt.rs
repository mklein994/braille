use anyhow::Context;

#[derive(Debug)]
pub struct Opt {
    /// The input's minimum value
    pub minimum: f64,
    /// The input's maximum value
    pub maximum: f64,
    /// How wide (in characters) the graph can be; defaults to the terminal width
    pub width: u16,
}

impl Opt {
    /// Build options out of a list of arguments passed on the command line
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Self> {
        Self::try_from_args(args).context("Usage: <STDIN> | braille <minimum> <maximum> [<width>]")
    }

    fn try_from_args(args: Vec<String>) -> anyhow::Result<Self> {
        anyhow::ensure!(
            args.len() == 2 || args.len() == 3,
            "Invalid number of arguments"
        );

        let mut args = args.into_iter();

        let minimum = args
            .next()
            .unwrap()
            .parse()
            .context("Invalid minimum value")?;
        let maximum = args
            .next()
            .unwrap()
            .parse()
            .context("Invalid maximum value")?;
        let width = args
            .next()
            .map(|x| x.parse())
            .transpose()
            .context("Invalid width value")?
            .or_else(|| {
                terminal_size::terminal_size().map(|(terminal_size::Width(width), _)| width)
            })
            .unwrap_or(80);

        anyhow::ensure!(minimum < maximum);

        Ok(Self {
            minimum,
            maximum,
            width,
        })
    }
}
