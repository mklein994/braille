use std::io::LineWriter;
use std::io::Write;

use crate::opt::ValueIter;
use crate::Config;
use crate::{ColumnGraphable, Graphable};

pub struct Columns {
    config: Config,
}

impl Graphable<Option<f64>> for Columns {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph(&self, lines: ValueIter<Option<f64>>) -> anyhow::Result<()> {
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);

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
            if right.is_none() {
                break;
            }

            let mut column = [vec![], vec![]];
            for (i, side) in [left, right].into_iter().enumerate() {
                if let Some(value) = side.transpose()?.and_then(|x| x.into_inner()).map(scale) {
                    column[i] = Self::into_dot_quads(value, zero);
                }
            }

            column_quads.push(column);
        }

        let mut writer = LineWriter::new(std::io::stdout());
        Self::into_braille_rows(&mut writer, &column_quads, usize::from(self.height()))?;

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

    fn print_graph(&self, lines: ValueIter<[Option<f64>; N]>) -> anyhow::Result<()> {
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);

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
            if right.is_none() {
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
                    column[i] = Self::into_dot_quads_from_array::<N>(value);
                }
            }

            column_quads.push(column);
        }

        let mut writer = LineWriter::new(std::io::stdout());
        Self::into_braille_rows(&mut writer, &column_quads, usize::from(self.height()))?;

        Ok(())
    }
}

impl ColumnGraphable<Option<f64>> for Columns {}

impl Columns {
    fn into_dot_quads(value: u16, zero: u16) -> Vec<[bool; 4]> {
        let prefix_length = usize::from(value.min(zero) - 1);
        let mut iter = vec![false; prefix_length];

        let stem_length = usize::from(value.abs_diff(zero) + 1);
        iter.resize(iter.len() + stem_length, true);

        let chunks = iter.chunks_exact(4);
        let tip = chunks.remainder().to_vec();
        let mut column: Vec<[bool; 4]> = chunks
            .into_iter()
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect();
        if !tip.is_empty() {
            column.push([
                tip[0],
                tip.get(1).copied().unwrap_or_default(),
                tip.get(2).copied().unwrap_or_default(),
                tip.get(3).copied().unwrap_or_default(),
            ]);
        }

        column
    }

    fn into_braille_rows<W: ?Sized + Write>(
        line_writer: &mut LineWriter<W>,
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
                    super::Lines::to_braille_char(raw_braille_char)
                )?;
            }

            writeln!(line_writer)?;
        }

        line_writer.flush()?;

        Ok(())
    }

    fn into_dot_quads_from_array<const N: usize>(line_set: [u16; N]) -> Vec<[bool; 4]> {
        assert_eq!(2, line_set.len(), "Not yet supported");
        let start = line_set[0];
        let end = line_set[1];
        let (start, end, filled) = if start <= end {
            (start, end, true)
        } else {
            (end, start, false)
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
        let tip = chunks.remainder().to_vec();
        let mut column: Vec<[bool; 4]> = chunks
            .into_iter()
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect();
        if !tip.is_empty() {
            column.push([
                tip[0],
                tip.get(1).copied().unwrap_or_default(),
                tip.get(2).copied().unwrap_or_default(),
                tip.get(3).copied().unwrap_or_default(),
            ]);
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
        #[rustfmt::skip]
        let input = vec![
            [
                vec![[ true,  true,  true,  true], [false, false, false, false]], // -3
                vec![[false,  true,  true,  true], [false, false, false, false]], // -2
            ],
            [
                vec![[false, false,  true,  true], [false, false, false, false]], // -1
                vec![[false, false, false,  true], [false, false, false, false]], //  0
            ],
            [
                vec![[false, false, false,  true], [ true, false, false, false]], //  1
                vec![[false, false, false,  true], [ true,  true, false, false]], //  2
            ],
            [
                vec![[false, false, false,  true], [ true,  true,  true, false]], //  3
                vec![[false, false, false,  true], [ true,  true,  true,  true]], //  4
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
            let mut line_writer = LineWriter::new(&mut buffer);
            Columns::into_braille_rows(&mut line_writer, &input, 2).unwrap();
        }
        let dot_pairs = String::from_utf8(buffer)
            .unwrap()
            .lines()
            .map(|line| line.chars().map(as_dot_pairs).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(expected, dot_pairs);
    }
}
