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
        let max = f64::from(opt.width * 8); // braille characters are 2 dots wide
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
            let tip = Self::BLOCKS[value.rem_euclid(8.) as usize];
            let full_block = *Self::BLOCKS.last().unwrap();
            let mut line = full_block.repeat(stem);
            line.push_str(tip);
            line
        } else {
            String::new()
        }
    }
}
