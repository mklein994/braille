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

fn print_help() {
    print_version();
    println!();
    println!(
        "\
USAGE:
    command | braille MINIMUM MAXIMUM [WIDTH]

ARGS:
    <MINIMUM>    The input's minimum value.
    <MAXIMUM>    The input's maximum value.
    <WIDTH>      The width of the graph. Defaults to the terminal width.

OPTIONS:
    -h, --help       Print this help text
    -V, --version    Print version information"
    );
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

impl Opt {
    /// Build options out of a list of arguments passed on the command line
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Self> {
        if args.is_empty() {
            print_help();
            std::process::exit(0);
        }

        for arg in &args {
            match arg.as_str() {
                "-h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                "-V" | "--version" => {
                    print_version();
                    std::process::exit(0);
                }
                _ => {}
            }
        }

        Self::try_from_args(args)
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
            .map(|x| {
                match x
                    .parse()
                    .map_err(|err: std::num::ParseIntError| anyhow::anyhow!(err))
                {
                    Ok(value) => {
                        anyhow::ensure!(value > 0, "Value must be at least 1. Given: {value}");
                        Ok(value)
                    }
                    Err(err) => Err(err),
                }
            })
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
