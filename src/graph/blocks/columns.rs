use std::io::{LineWriter, Write};
use std::str::FromStr;

use crate::graph::{ColumnGraphable, Graphable};
use crate::opt::ValueIter;
use crate::Config;
use crate::InputLine;

pub struct Columns {
    config: Config,
}

impl From<Config> for Columns {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl Columns
where
    Self: ColumnGraphable<Option<f64>>,
{
    // // FOR DEBUGGING
    // const BLOCKS: [&'static str; 9] = [
    //     "0", // ' ' (space)
    //     "1", // ▁
    //     "2", // ▂
    //     "3", // ▃
    //     "4", // ▄
    //     "5", // ▅
    //     "6", // ▆
    //     "7", // ▇
    //     "8", // █
    // ];

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

impl Graphable<Option<f64>> for Columns
where
    InputLine<Option<f64>>: FromStr,
{
    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<Option<f64>>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let lines: Vec<_> = lines.into_iter().collect();
        let minimum = self.minimum();
        let maximum = self.maximum();

        let min = 1.;
        let max = f64::from(self.height() * 8);
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

        for row in (0..usize::from(self.height())).rev() {
            for column in &columns {
                write!(writer, "{}", column.get(row).unwrap_or(&" "))?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }
}

impl ColumnGraphable<Option<f64>> for Columns {}
