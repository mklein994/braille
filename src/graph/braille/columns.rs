use crate::graph::{BarGraphable, Graphable};
use crate::Config;

pub struct Columns {
    config: Config,
}

impl Graphable for Columns {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl BarGraphable for Columns {
    fn print_bars(&self, _lines: Vec<crate::LineResult>) -> anyhow::Result<()> {
        todo!()
    }
}
