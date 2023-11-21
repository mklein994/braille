use crate::graph::{ColumnGraphable, Graphable};
use crate::{Config, LineResult};

pub struct Columns {
    config: Config,
}

impl Columns
where
    Self: ColumnGraphable,
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

    fn calculate_column(value: Option<f64>) -> Vec<&'static str> {
        if let Some(value) = value {
            let stem = (value / 8.).trunc() as usize;
            let tip = (value % 8.) as usize;
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

impl Graphable for Columns {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl ColumnGraphable for Columns {
    fn print_columns(&self, lines: Vec<LineResult>) -> anyhow::Result<()> {
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
            let column = Self::calculate_column(line?.map(scale));
            columns.push(column);
        }

        for row in (0..usize::from(self.height())).rev() {
            for column in &columns {
                print!("{}", column.get(row).unwrap_or(&" "));
            }
            println!();
        }

        Ok(())
    }
}
