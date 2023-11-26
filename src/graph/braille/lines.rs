//! Turn a series of numbers into a braille graph.
//!
//! # Example
//!
//! ## Sine graph
//!
//! ```console
//! $ awk 'BEGIN { for (i = 0; i < 20; i++) { print sin(i / 3); } }' | braille -1 1 4
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
//! $ seq -4 3 | braille -4 3 7
//! ⠉⠛⠿⣿
//! ⠀⠀⠀⢸⣶⣤⣀
//! ```

use crate::graph::{BarGraphable, Graphable};
use crate::input::{InputLine, InputLineSinglable};
use crate::opt::{Config, GraphStyle, ValueIter};

#[derive(Debug)]
pub struct Lines {
    config: Config,
}

impl Graphable<Option<f64>> for Lines {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph(&self, input_lines: ValueIter<Option<f64>>) -> anyhow::Result<()> {
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
                    .map(|value| Self::into_dot_pairs(value, zero, style))
                {
                    *buffer_line = new_line;
                }
            }

            if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
                let transposed = Self::transpose_row(&buffer);
                let braille_char = Self::to_braille_char_row(&transposed);
                println!("{braille_char}");
            }

            buffer.fill(vec![]);
        }

        Ok(())
    }
}

impl BarGraphable<Option<f64>> for Lines {}

impl Lines {
    /// Turn a value into its representation of braille dots for that row
    ///
    /// # Example:
    ///
    /// Let's say the width is 4 characters wide, and the input is all integers in the range `[-3, 4]`
    /// incrementing by one. The braille pattern would look like this:
    ///
    /// ```plain
    /// ⠙⢿
    /// ⠀⢸⣷⣄
    /// ```
    ///
    /// Depending on your font, this may appear with gaps between lines or characters. In my
    /// experience, [Cascadia Code](https://github.com/microsoft/cascadia-code) looks really good.
    ///
    /// Breaking this down, let's look at these lines in particular:
    ///
    /// | Value | ASCII braille | Notes            |
    /// |------:|:--------------|------------------|
    /// |    -3 | `** **`       |                  |
    /// |    -2 | `-* **`       | 1st example row  |
    /// |    -1 | `-- **`       |                  |
    /// |     0 | `-- -*`       |                  |
    /// |       |               | _next character_ |
    /// |     1 | `-- -* *- --` |                  |
    /// |     2 | `-- -* ** --` |                  |
    /// |     3 | `-- -* ** *-` | 2nd example row  |
    /// |     4 | `-- -* ** **` |                  |
    ///
    /// In the first example row, the value -2 translates to dot 4. This means dots 2 through 4 are
    /// filled, and any before that are blank. Since the value is below zero, it looks like this:
    ///
    /// ```
    /// //   ┌──── -2 (dot 2)
    /// //  -* **
    /// //      └─  0 (dot 4)
    /// assert_eq!(vec![[false, true], [true, true]], braille::BrailleLines::into_dot_pairs(2, 4, braille::GraphStyle::default()));
    /// ```
    ///
    /// In the second example row, the input value is 3, which translates to dot 7. The value is above
    /// zero, and looks like this:
    ///
    /// ```
    /// //          ┌── 3 (dot 7)
    /// // -- -* ** *-
    /// //     └─────── 0 (dot 4)
    /// assert_eq!(
    ///     vec![[false, false], [false, true], [true, true], [true, false]],
    ///     braille::BrailleLines::into_dot_pairs(7, 4,
    ///     braille::GraphStyle::default())
    /// );
    /// ```
    ///
    /// Looking at each example line more closely, we can see it broken down into parts:
    ///
    /// ```plain
    ///     ┌───── prefix
    /// 1.  -* **
    ///      └─┴┴─ stem
    ///
    ///     ┌┬─┬──────── prefix
    ///     ││ │     ┌┬─ tip
    /// 2.  -- -* ** *-
    ///         └─┴┴──── stem
    /// ```
    #[must_use]
    pub fn into_dot_pairs(value: u16, zero: u16, style: GraphStyle) -> Vec<[bool; 2]> {
        let prefix_length = usize::from(value.min(zero) - 1);
        let mut iter = vec![false; prefix_length];

        let stem_length = usize::from(value.abs_diff(zero));
        for i in 0..=stem_length {
            if i == stem_length {
                iter.push(true);
            } else {
                iter.push(match style {
                    GraphStyle::Auto | GraphStyle::Filled => true,
                    GraphStyle::Line => false,
                });
            }
        }

        let chunks = iter.chunks_exact(2);
        let tip = chunks.remainder().to_vec();
        let mut row: Vec<[bool; 2]> = chunks
            .into_iter()
            .map(|chunk| [chunk[0], chunk[1]])
            .collect();
        if !tip.is_empty() {
            row.push([tip[0], tip.get(1).copied().unwrap_or_default()]);
        }
        row
    }

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
        let tip = chunks.remainder().to_vec();
        let mut row: Vec<[bool; 2]> = chunks
            .into_iter()
            .map(|chunk| [chunk[0], chunk[1]])
            .collect();
        if !tip.is_empty() {
            row.push([tip[0], tip.get(1).copied().unwrap_or_default()]);
        }
        row
    }

    /// Turns an array of dot pairs into a braille character.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(
    ///     braille::BrailleLines::to_braille_char([
    ///         [true, true],
    ///         [false, true],
    ///         [true, false],
    ///         [true, true],
    ///     ]),
    ///     '⣝'
    /// );
    /// ```
    ///
    /// See also: <https://en.wikipedia.org/wiki/Braille_Patterns>
    #[must_use]
    pub fn to_braille_char(dot_pairs: [[bool; 2]; 4]) -> char {
        // Turn this:
        //
        // ```plain
        // [
        //   [0, 3],
        //   [1, 4],
        //   [2, 5],
        //   [6, 7],
        // ]
        // ```
        //
        // into this:
        //
        // ```plain
        // [0, 1, 2, 3, 4, 5, 6, 7]
        // ```
        let bits = [
            dot_pairs[0][0],
            dot_pairs[1][0],
            dot_pairs[2][0],
            dot_pairs[0][1],
            dot_pairs[1][1],
            dot_pairs[2][1],
            dot_pairs[3][0],
            dot_pairs[3][1],
        ];

        let mut block = 0x2800_u32; // empty braille character

        for (index, bit) in bits.iter().enumerate() {
            if *bit {
                let position = u32::try_from(index).unwrap();
                block += (2_u32).pow(position);
            }
        }

        char::from_u32(block).expect("braille character not valid")
    }

    /// Render a list of braille dot blocks as a string
    #[must_use]
    pub fn to_braille_char_row(transposed: &[[[bool; 2]; 4]]) -> String {
        let mut line = String::new();
        for dot_pairs in transposed {
            let braille_char = Self::to_braille_char(*dot_pairs);

            line.push(braille_char);
        }
        line
    }

    /// Turn a list of braille dot pairs into a list of braille dot blocks
    #[must_use]
    pub fn transpose_row(input_row: &[Vec<[bool; 2]>; 4]) -> Vec<[[bool; 2]; 4]> {
        let longest = input_row.iter().map(Vec::len).max().unwrap();

        let mut output_row: Vec<[[bool; 2]; 4]> = vec![];
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

    fn print_graph(&self, input_lines: ValueIter<[Option<f64>; N]>) -> anyhow::Result<()> {
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
                        if x.as_single_iter().all(|x| x.is_none()) {
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
                let braille_char = Self::to_braille_char_row(&transposed);
                println!("{braille_char}");
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
            Lines::into_dot_pairs(5, 5, GraphStyle::default())
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
