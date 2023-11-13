#[derive(Debug)]
pub struct Opt {
    pub minimum: f64,
    pub maximum: f64,
    pub width: u16,
}

impl Opt {
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Self> {
        let mut args = args.into_iter();

        let minimum = args.next().and_then(|x| x.parse().ok()).ok_or_else(|| {
            anyhow::anyhow!("Usage: <STDIN> | braille <minimum> <maximum> [<width>]")
        })?;
        let maximum = args
            .next()
            .map(|x| x.parse::<f64>())
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("missing maximum argument"))?;
        let width = args
            .next()
            .map(|x| x.parse())
            .transpose()?
            .or_else(|| {
                terminal_size::terminal_size().map(|(terminal_size::Width(width), _)| width)
            })
            .unwrap_or(80);

        assert!(minimum < maximum);

        Ok(Self {
            minimum,
            maximum,
            width,
        })
    }
}
