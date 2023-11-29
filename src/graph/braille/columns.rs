use std::io::{LineWriter, Write};

use super::Brailleish;
use super::Char as BrailleChar;
use crate::opt::ValueIter;
use crate::Config;
use crate::GraphStyle;
use crate::InputLine;
use crate::{ColumnGraphable, Graphable};

pub struct Columns {
    config: Config,
}

impl Brailleish<4> for Columns {}

impl Graphable<Option<f64>> for Columns {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<Option<f64>>,
        writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);
        let style = <Self as Graphable<Option<f64>, Config>>::style(self);

        let min = 1;
        let max = self.height() * 4;
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

        let mut input_lines = lines.into_iter();

        let mut column_quads = vec![];

        loop {
            let left = input_lines.next();
            let right = input_lines.next();
            if left.is_none() {
                break;
            }

            let mut column = [vec![], vec![]];
            for (i, side) in [left, right].into_iter().enumerate() {
                if let Some(value) = side.transpose()?.and_then(InputLine::into_inner).map(scale) {
                    column[i] = Self::into_dot_groups(value, zero, style);
                }
            }

            column_quads.push(column);
        }

        Self::into_braille_rows(writer, &column_quads, usize::from(self.height()))?;

        Ok(())
    }
}

impl<const N: usize> Graphable<[Option<f64>; N]> for Columns {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<[Option<f64>; N]>,
        writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);
        let style = <Self as Graphable<Option<f64>, Config>>::style(self);

        let min = 1;
        let max = self.height() * 4;
        let slope = f64::from(max - min) / (maximum - minimum);
        let scale = |value: f64| {
            assert!(
                value >= minimum && value <= maximum,
                "value out of bounds: {value} [{minimum}, {maximum}]"
            );
            min + (slope * (value - minimum)).round() as u16
        };

        let mut input_lines = lines.into_iter();

        let mut column_quads = vec![];

        loop {
            let left = input_lines.next();
            let right = input_lines.next();
            if left.is_none() {
                break;
            }

            let mut column = [vec![], vec![]];
            for (i, side) in [left, right].into_iter().enumerate() {
                if let Some(value) = side.transpose()?.map(|input_line_value| {
                    input_line_value
                        .into_iter()
                        .map(|x| x.map(scale).unwrap_or_default())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                }) {
                    column[i] = Self::into_dot_quads_from_array::<N>(value, style);
                }
            }

            column_quads.push(column);
        }

        Self::into_braille_rows(writer, &column_quads, usize::from(self.height()))?;

        Ok(())
    }
}

impl ColumnGraphable<Option<f64>> for Columns {}

impl Columns {
    fn into_braille_rows<W: Write>(
        mut line_writer: LineWriter<W>,
        column_quads: &[[Vec<[bool; 4]>; 2]],
        height: usize,
    ) -> std::io::Result<()> {
        for row_index in (0..height).rev() {
            for col in column_quads {
                let mut raw_braille_char = [[false; 2]; 4];
                for (character_row, pair) in raw_braille_char.iter_mut().rev().enumerate() {
                    let left = col
                        .first()
                        .and_then(|c| c.get(row_index))
                        .and_then(|c| c.get(character_row))
                        .copied()
                        .unwrap_or_default();
                    let right = col
                        .last()
                        .and_then(|c| c.get(row_index))
                        .and_then(|c| c.get(character_row))
                        .copied()
                        .unwrap_or_default();
                    *pair = [left, right];
                }

                write!(
                    line_writer,
                    "{}",
                    BrailleChar::new(raw_braille_char).as_char()
                )?;
            }

            writeln!(line_writer)?;
        }

        Ok(())
    }

    fn into_dot_quads_from_array<const N: usize>(
        line_set: [u16; N],
        style: GraphStyle,
    ) -> Vec<[bool; 4]> {
        assert_eq!(2, line_set.len(), "Not yet supported");
        let start = line_set[0];
        let end = line_set[1];
        let filled = match (start, end, style) {
            (start, end, GraphStyle::Auto) => start <= end,
            (_, _, GraphStyle::Line) => false,
            (_, _, GraphStyle::Filled) => true,
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

        let chunks = iter.chunks_exact(4);
        let mut tip = chunks.remainder().to_vec();
        let mut column: Vec<[bool; 4]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(4, false);
            column.push(tip.try_into().unwrap());
        }

        column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_dot_pairs(raw: char) -> [[bool; 2]; 4] {
        let bits = u32::from(raw) - 0x2800;
        [
            [bits & 2_u32.pow(0) != 0, bits & 2_u32.pow(3) != 0],
            [bits & 2_u32.pow(1) != 0, bits & 2_u32.pow(4) != 0],
            [bits & 2_u32.pow(2) != 0, bits & 2_u32.pow(5) != 0],
            [bits & 2_u32.pow(6) != 0, bits & 2_u32.pow(7) != 0],
        ]
    }

    #[test]
    fn transpose_n3_4_2() {
        //  4  -- -- -- -*
        //  3  -- -- -- **
        //  2  -- -- -* **
        //  1  -- -- ** **
        //
        //  0  ** ** ** **
        // -1  ** *- -- --
        // -2  ** -- -- --
        // -3  *- -- -- --
        let input = vec![
            [
                vec![[true, true, true, true], [false, false, false, false]], // -3
                vec![[false, true, true, true], [false, false, false, false]], // -2
            ],
            [
                vec![[false, false, true, true], [false, false, false, false]], // -1
                vec![[false, false, false, true], [false, false, false, false]], //  0
            ],
            [
                vec![[false, false, false, true], [true, false, false, false]], //  1
                vec![[false, false, false, true], [true, true, false, false]],  //  2
            ],
            [
                vec![[false, false, false, true], [true, true, true, false]], //  3
                vec![[false, false, false, true], [true, true, true, true]],  //  4
            ],
        ];

        #[rustfmt::skip]
        let expected = vec![
            vec![
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
                    [false, false],
                    [false, false],
                    [false,  true],
                    [ true,  true],
                ],
                [
                    [false,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
            ],
            vec![
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true, false],
                ],
                [
                    [ true,  true],
                    [ true, false],
                    [false, false],
                    [false, false],
                ],
                [
                    [ true,  true],
                    [false, false],
                    [false, false],
                    [false, false],
                ],
                [
                    [ true,  true],
                    [false, false],
                    [false, false],
                    [false, false],
                ],
            ],
        ];

        let mut buffer = vec![];
        {
            let line_writer = LineWriter::new(&mut buffer);
            Columns::into_braille_rows(line_writer, &input, 2).unwrap();
        }
        let dot_pairs = String::from_utf8(buffer)
            .unwrap()
            .lines()
            .map(|line| line.chars().map(as_dot_pairs).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(expected, dot_pairs);
    }

    #[test]
    fn transpose_all_max_2() {
        // 7  ** ** ** **
        // 6  ** ** ** **
        // 5  ** ** ** **
        // 4  ** ** ** **
        //
        // 3  ** ** ** **
        // 2  ** ** ** **
        // 1  ** ** ** **
        // 0  ** ** ** **
        #[rustfmt::skip]
        let input = vec![
            [
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
            ],
            [
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
            ],
            [
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
            ],
            [
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
                vec![[ true,  true,  true,  true], [ true,  true,  true,  true]],
            ],
        ];

        #[rustfmt::skip]
        let expected = vec![
            vec![
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
            ],
            vec![
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
                [
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                    [ true,  true],
                ],
            ],
        ];

        let mut buffer = vec![];
        {
            let line_writer = LineWriter::new(&mut buffer);
            Columns::into_braille_rows(line_writer, &input, 2).unwrap();
        }
        let dot_pairs = String::from_utf8(buffer)
            .unwrap()
            .lines()
            .map(|line| line.chars().map(as_dot_pairs).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(expected, dot_pairs);
    }
}
