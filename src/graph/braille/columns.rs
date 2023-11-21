use crate::graph::{ColumnGraphable, Graphable};
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

impl ColumnGraphable for Columns {
    fn print_columns(&self, _lines: Vec<crate::LineResult>) -> anyhow::Result<()> {
        todo!()
    }
}
