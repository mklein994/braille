//! Turn a series of numbers into a braille graph.
//!
//! # Example
//!
//! ## Sine graph
//!
//! ```console
//! $ awk 'BEGIN { for (i = 0; i < 20; i++) { print sin(i / 3); } }' | braille 4
//! ⠀⠀⠀⠀⠀⣷⣶⣤⣀
//! ⠀⠀⠀⠀⠀⣿⣿⣿⡿⠟
//! ⠀⠀⢀⣀⣤⡟⠉⠁
//! ⣴⣿⣿⣿⣿⡇
//! ⠀⠉⠛⠻⠿⡇
//! ```
//!
//! ## Simple number sequence
//!
//! ```console
//! $ seq -4 3 | braille 7
//! ⠉⠛⠿⣿
//! ⠀⠀⠀⢸⣶⣤⣀
//! ```

use std::io::{LineWriter, Write};

use super::Brailleish;
use super::Char as BrailleChar;
use crate::graph::{BarGraphable, Graphable};
use crate::opt::{Config, GraphStyle, ValueIter};
use crate::{InputLine, InputLineSinglable};

#[derive(Debug)]
pub struct Lines {
    config: Config,
}

impl Brailleish<2> for Lines {}

impl Graphable<Option<f64>> for Lines {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        input_lines: ValueIter<Option<f64>>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let mut input_lines = input_lines.into_iter();
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);

        let min = 1; // reserve an empty line for null values
        let max = self.width() * 2; // braille characters are 2 dots wide
        let slope = f64::from(max - min) / (maximum - minimum);
        let scale = |value: f64| {
            assert!(
                value >= minimum && value <= maximum,
                "value out of bounds: {value} [{minimum}, {maximum}]"
            );
            min + (slope * (value - minimum)).round() as u16
        };

        // Clamp where 0 would fit to be inside the output range
        let zero = if minimum > 0. {
            min
        } else if maximum < 0. {
            max
        } else {
            scale(0.)
        };

        let style = <Self as Graphable<Option<f64>, Config>>::style(self);

        // Each braille character is 4 dots tall
        let mut buffer = [vec![], vec![], vec![], vec![]];
        let mut has_more_lines = true;
        while has_more_lines {
            for buffer_line in &mut buffer {
                let input_line = input_lines.next();
                if input_line.is_none() {
                    has_more_lines = false;
                }

                if let Some(new_line) = input_line
                    .transpose()?
                    .and_then(InputLine::into_inner)
                    .map(scale)
                    .map(|value| Self::into_dot_groups(value, zero, style))
                {
                    *buffer_line = new_line;
                }
            }

            if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
                let transposed = Self::transpose_row(&buffer);
                let braille_line = transposed
                    .into_iter()
                    .map(|x| BrailleChar::new(x).as_char())
                    .collect::<String>();
                writeln!(writer, "{braille_line}")?;
            }

            buffer.fill(vec![]);
        }

        Ok(())
    }
}

impl BarGraphable<Option<f64>> for Lines {}

impl Lines {
    #[must_use]
    pub fn into_dot_array_pairs<const N: usize>(
        line_set: [u16; N],
        style: GraphStyle,
    ) -> Vec<[bool; 2]> {
        assert_eq!(2, line_set.len(), "Not yet supported");
        let start = line_set[0];
        let end = line_set[1];

        let filled = match (start, end, style) {
            (_, _, GraphStyle::Line) => false,
            (_, _, GraphStyle::Filled) => true,
            (start, end, GraphStyle::Auto) => start <= end,
        };

        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };

        let prefix_length = usize::from(start - 1);
        let mut iter = vec![false; prefix_length];

        let stem_length = usize::from(start.abs_diff(end));
        for i in 0..=stem_length {
            if i == 0 || i == stem_length {
                iter.push(true);
            } else {
                iter.push(filled);
            }
        }

        let chunks = iter.chunks_exact(2);
        let mut tip = chunks.remainder().to_vec();
        let mut row: Vec<[bool; 2]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(2, false);
            row.push(tip.try_into().unwrap());
        }
        row
    }

    /// Turn a list of braille dot pairs into a list of braille dot blocks
    #[must_use]
    pub fn transpose_row(input_row: &[Vec<[bool; 2]>; 4]) -> Vec<[[bool; 2]; 4]> {
        let longest = input_row.iter().map(Vec::len).max().unwrap();

        let mut output_row = vec![];
        for column in 0..longest {
            let mut braille_character = [[false, false]; 4];

            for (row_index, row) in input_row.iter().enumerate() {
                if let Some(row_column) = row.get(column) {
                    braille_character[row_index] = *row_column;
                }
            }

            if column < longest - 1 || braille_character.into_iter().flatten().any(|x| x) {
                output_row.push(braille_character);
            }
        }

        output_row
    }
}

impl<const N: usize> Graphable<[Option<f64>; N]> for Lines {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        input_lines: ValueIter<[Option<f64>; N]>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let mut input_lines = input_lines.into_iter();
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);

        let min = 1; // reserve an empty line for null values
        let max = self.width() * 2; // braille characters are 2 dots wide
        let slope = f64::from(max - min) / (maximum - minimum);
        let scale = |value: f64| {
            assert!(
                value >= minimum && value <= maximum,
                "value out of bounds: {value} [{minimum}, {maximum}]"
            );
            min + (slope * (value - minimum)).round() as u16
        };

        // Each braille character is 4 dots tall
        let mut buffer = [vec![], vec![], vec![], vec![]];
        let mut has_more_lines = true;
        while has_more_lines {
            for buffer_line in &mut buffer {
                let input_line = input_lines.next();
                if input_line.is_none() {
                    has_more_lines = false;
                }

                if let Some(new_line) = input_line
                    .transpose()?
                    .and_then(|x| {
                        if x.as_single_iter().all(Option::is_none) {
                            None
                        } else {
                            let line = x.into_iter().map(|x| scale(x.unwrap())).collect::<Vec<_>>();
                            Some(<[_; N]>::try_from(line).unwrap())
                        }
                    })
                    .map(|x| {
                        Self::into_dot_array_pairs(
                            x,
                            <Self as Graphable<Option<f64>, Config>>::style(self),
                        )
                    })
                {
                    *buffer_line = new_line;
                }
            }

            if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
                let transposed = Self::transpose_row(&buffer);
                let braille_line = transposed
                    .into_iter()
                    .map(|x| BrailleChar::new(x).as_char())
                    .collect::<String>();
                writeln!(writer, "{braille_line}")?;
            }

            buffer.fill(vec![]);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_dot_pairs() {
        assert_eq!(
            vec![[false, false], [false, false], [true, false]],
            Lines::into_dot_groups(5, 5, GraphStyle::default())
        );
    }

    #[test]
    fn test_transpose_row_line_1() {
        #[rustfmt::skip]
        let input = [
            vec![[ true,  true], [ true, true], [true, false]],
            vec![[false,  true], [ true, true], [true, false]],
            vec![[false, false], [ true, true], [true, false]],
            vec![[false, false], [false, true], [true, false]],
        ];

        #[rustfmt::skip]
        let expected = vec![
            [
                [ true,  true],
                [false,  true],
                [false, false],
                [false, false],
            ],
            [
                [ true,  true],
                [ true,  true],
                [ true,  true],
                [false,  true],
            ],
            [
                [ true, false],
                [ true, false],
                [ true, false],
                [ true, false],
            ],
        ];
        let actual = Lines::transpose_row(&input);
        for (ex, act) in expected.iter().zip(actual.iter()) {
            eprintln!("{ex:?}");
            eprintln!("{act:?}");
            eprintln!();
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_transpose_row_line_2() {
        #[rustfmt::skip]
        let input = [
            vec![[false, false], [false, false], [true, false]               ],
            vec![[false, false], [false, false], [true,  true]               ],
            vec![[false, false], [false, false], [true,  true], [true, false]],
            vec![[false, false], [false, false], [true,  true], [true,  true]],
        ];

        #[rustfmt::skip]
        let expected = vec![
            [
                [false, false],
                [false, false],
                [false, false],
                [false, false],
            ],
            [
                [false, false],
                [false, false],
                [false, false],
                [false, false],
            ],
            [
                [ true, false],
                [ true,  true],
                [ true,  true],
                [ true,  true]
            ],
            [
                [false, false],
                [false, false],
                [ true, false],
                [ true,  true],
            ],
        ];

        assert_eq!(expected, Lines::transpose_row(&input));
    }
}
