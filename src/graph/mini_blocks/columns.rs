use super::Char;
use crate::graph::braille::Brailleish;
use crate::opt::ValueIter;
use crate::Config;
use crate::GraphStyle;
use crate::InputLine;
use crate::{ColumnGraphable, Graphable};
use std::io::{LineWriter, Write};

pub struct Columns {
    config: Config,
}

impl ColumnGraphable<Option<f64>> for Columns {}

impl Brailleish<2> for Columns {}

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
        let max = self.height() * 2;
        let scale = |value: f64| Self::scale(value, minimum, maximum, min, max);

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
        let max = self.height() * 2;
        let scale = |value: f64| Self::scale(value, minimum, maximum, min, max);

        let mut input_lines = lines.into_iter();

        let mut column_pairs = vec![];

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
                    column[i] = Self::into_dot_pairs_from_array::<N>(value, style);
                }
            }

            column_pairs.push(column);
        }

        Self::into_braille_rows(writer, &column_pairs, usize::from(self.height()))?;

        Ok(())
    }
}

impl Columns {
    fn into_braille_rows<W: Write>(
        mut line_writer: LineWriter<W>,
        column_pairs: &[[Vec<[bool; 2]>; 2]],
        height: usize,
    ) -> std::io::Result<()> {
        for row_index in (0..height).rev() {
            for col in column_pairs {
                let mut raw_braille_char = [[false; 2]; 2];
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

                write!(line_writer, "{}", Char::new(raw_braille_char))?;
            }

            writeln!(line_writer)?;
        }

        Ok(())
    }

    fn into_dot_pairs_from_array<const N: usize>(
        line_set: [u16; N],
        style: GraphStyle,
    ) -> Vec<[bool; 2]> {
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

        let chunks = iter.chunks_exact(2);
        let mut tip = chunks.remainder().to_vec();
        let mut column: Vec<[bool; 2]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(2, false);
            column.push(tip.try_into().unwrap());
        }

        column
    }
}
