use super::ColumnGraphable;
use crate::Opt;

#[derive(Debug, Default)]
pub struct ColBlock;

impl ColumnGraphable for ColBlock {
    type Item = Option<f64>;
    type Error = std::num::ParseFloatError;

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
    fn print_lines<I: Iterator<Item = Result<Self::Item, Self::Error>>>(
        opt: &Opt,
        input_lines: I,
    ) -> anyhow::Result<()> {
        let min = 1.; // reserve an empty line for null values
        let max = f64::from(opt.width() * 8); // braille characters are 2 dots wide
        let slope = (max - min) / (opt.maximum - opt.minimum);
        let scale = |value: f64| {
            assert!(
                value >= opt.minimum && value <= opt.maximum,
                "value out of bounds: {value} [{}, {}]",
                opt.minimum,
                opt.maximum
            );
            min + slope * (value - opt.minimum)
        };

        for line in input_lines {
            println!("{}", Self::print_line(line?.map(scale)));
        }

        Ok(())
    }
}

impl ColBlock {
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
                assert_eq!($expected, ColBlock::print_line($value));
            }
        };
    }

    t!(colblock_print_line_none, "", None);
    t!(colblock_print_line_0, "", Some(0.));
    t!(colblock_print_line_1, "▏", Some(1.));
    t!(colblock_print_line_2, "▎", Some(2.));
    t!(colblock_print_line_3, "▍", Some(3.));
    t!(colblock_print_line_4, "▌", Some(4.));
    t!(colblock_print_line_5, "▋", Some(5.));
    t!(colblock_print_line_6, "▊", Some(6.));
    t!(colblock_print_line_7, "▉", Some(7.));
    t!(colblock_print_line_8, "█", Some(8.));
}