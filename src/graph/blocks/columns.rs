use crate::graph::{ColumnGraphable, Graphable};
use crate::opt::Config;
use crate::LineResult;

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
    /// ```plain
    /// █
    /// ▉
    /// ▊
    /// ▋
    /// ▌
    /// ▍
    /// ▎
    /// ▏
    ///   (space)
    /// ```
    fn print_lines(&self, input_lines: impl Iterator<Item = LineResult>) -> anyhow::Result<()> {
        let minimum = self.minimum();
        let maximum = self.maximum();

        let min = 1.; // reserve an empty line for null values
        let max = f64::from(self.width() * 8); // braille characters are 2 dots wide
        let slope = (max - min) / (maximum - minimum);
        let scale = |value: f64| {
            assert!(
                value >= minimum && value <= maximum,
                "value out of bounds: {value} [{minimum}, {maximum}]"
            );
            min + slope * (value - minimum)
        };

        for line in input_lines {
            println!("{}", Self::print_line(line?.map(scale)));
        }

        Ok(())
    }
}

impl Columns {
    const BLOCKS: [&'static str; 9] = [
        "",         // ' '
        "\u{258f}", // ▏
        "\u{258e}", // ▎
        "\u{258d}", // ▍
        "\u{258c}", // ▌
        "\u{258b}", // ▋
        "\u{258a}", // ▊
        "\u{2589}", // ▉
        "\u{2588}", // █
    ];

    fn print_line(value: Option<f64>) -> String {
        if let Some(value) = value {
            let stem = (value / 8.).trunc() as usize;
            let tip = Self::BLOCKS[(value % 8.) as usize];
            let full_block = Self::BLOCKS.last().unwrap();
            let mut line = full_block.repeat(stem);
            line.push_str(tip);
            line
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($name:ident, $expected:literal, $value:expr) => {
            #[test]
            fn $name() {
                assert_eq!($expected, Columns::print_line($value));
            }
        };
    }

    t!(columns_print_line_none, "", None);
    t!(columns_print_line_0, "", Some(0.));
    t!(columns_print_line_1, "▏", Some(1.));
    t!(columns_print_line_2, "▎", Some(2.));
    t!(columns_print_line_3, "▍", Some(3.));
    t!(columns_print_line_4, "▌", Some(4.));
    t!(columns_print_line_5, "▋", Some(5.));
    t!(columns_print_line_6, "▊", Some(6.));
    t!(columns_print_line_7, "▉", Some(7.));
    t!(columns_print_line_8, "█", Some(8.));
}
