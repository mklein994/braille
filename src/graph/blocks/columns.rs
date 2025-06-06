use std::io::{LineWriter, Write};

use crate::Config;
use crate::graph::{ColumnGraphable, Graphable};
use crate::opt::ValueIter;

pub struct Columns {
    config: Config,
}

impl From<Config> for Columns {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl ColumnGraphable<Option<f64>> for Columns {}
impl Graphable<Option<f64>> for Columns {
    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<Option<f64>>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let lines: Vec<_> = lines.into_iter().collect();
        let minimum = <Self as Graphable<Option<f64>>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>>>::maximum(self);
        let height = <Self as ColumnGraphable<Option<f64>>>::height(self);

        let min = 1.;
        let max = f64::from(height * 8);
        let slope = (max - min) / (maximum - minimum);

        let mut columns = Vec::with_capacity(lines.len());

        let scale = |value: f64| {
            assert!(
                value >= minimum && value <= maximum,
                "value out of bounds: {value} [{minimum}, {maximum}]"
            );
            min + slope * (value - minimum)
        };

        for line in lines {
            let column = Self::calculate_column(line?.into_inner().map(scale));
            columns.push(column);
        }

        for row in (0..usize::from(height)).rev() {
            for column in &columns {
                write!(writer, "{}", column.get(row).unwrap_or(&" "))?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }
}

impl Columns {
    const BLOCKS: [&'static str; 9] = [
        " ",        // ' ' (space)
        "\u{2581}", // ▁
        "\u{2582}", // ▂
        "\u{2583}", // ▃
        "\u{2584}", // ▄
        "\u{2585}", // ▅
        "\u{2586}", // ▆
        "\u{2587}", // ▇
        "\u{2588}", // █
    ];

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn calculate_column(value: Option<f64>) -> Vec<&'static str> {
        if let Some(value) = value {
            let stem = (value / 8.).trunc() as usize;
            let tip = (value % 8.).trunc() as usize;
            let full_block = *Self::BLOCKS.last().unwrap();
            let mut column = vec![full_block; stem];
            if tip > 0 {
                column.push(Self::BLOCKS[tip]);
            }

            column
        } else {
            vec![" "]
        }
    }
}
