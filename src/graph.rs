pub mod blocks;
pub mod braille;
mod dot_plotter;
pub mod mini_blocks;
pub mod octants;
pub mod sextants;

use std::io::{LineWriter, Write};
use std::str::FromStr;

use crate::InputLine;
use crate::opt::{Config, Configurable, GraphStyle, ValueIter};

pub trait Graphable<T, Conf: Configurable = Config>
where
    Self: From<Conf>,
    InputLine<T>: FromStr,
{
    fn config(&self) -> &Conf;

    fn minimum(&self) -> f64 {
        self.config().minimum()
    }

    fn maximum(&self) -> f64 {
        self.config().maximum()
    }

    fn style(&self) -> GraphStyle {
        self.config().style()
    }

    fn print_graph<W: Write>(
        &self,
        lines: ValueIter<T>,
        writer: LineWriter<W>,
    ) -> anyhow::Result<()>;
}

pub trait BarGraphable<T>: Graphable<T>
where
    InputLine<T>: FromStr,
{
    fn width(&self) -> u16 {
        self.config().size()
    }
}

pub trait ColumnGraphable<T>: Graphable<T>
where
    InputLine<T>: FromStr,
{
    fn height(&self) -> u16 {
        self.config().size()
    }
}

pub trait RowBuildable {
    /// Turn M rows of dot sets N wide each into a list of `NxM` dot blocks
    #[must_use]
    fn assemble_row<const N: usize, const M: usize>(
        input_row: &[Vec<[bool; N]>; M],
    ) -> Vec<[[bool; N]; M]> {
        let longest = input_row.iter().map(Vec::len).max().unwrap();

        let mut output_row = vec![];
        for column in 0..longest {
            let mut character = [[false; N]; M];

            for (row_index, row) in input_row.iter().enumerate() {
                if let Some(row_column) = row.get(column) {
                    character[row_index] = *row_column;
                }
            }

            if column < longest - 1 || character.into_iter().flatten().any(|x| x) {
                output_row.push(character);
            }
        }

        output_row
    }
}

pub trait DotArrayable {
    #[must_use]
    fn into_dot_array_groups<const N: usize, const M: usize>(
        line_set: [u16; N],
        style: GraphStyle,
    ) -> Vec<[bool; M]> {
        assert_eq!(
            2,
            line_set.len(),
            "Plotting more than 2 series at a time is not supported"
        );
        let start = line_set[0];
        let end = line_set[1];

        let filled = match style {
            GraphStyle::Auto => start <= end,
            GraphStyle::Filled => true,
            GraphStyle::Line => false,
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

        let chunks = iter.chunks_exact(M);
        let mut tip = chunks.remainder().to_vec();
        let mut row: Vec<[bool; M]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(M, false);
            row.push(tip.try_into().unwrap());
        }
        row
    }

    #[must_use]
    fn into_dot_array_groups_variable<const M: usize>(
        line_set: &[u16],
        style: GraphStyle,
    ) -> Vec<[bool; M]> {
        assert!(
            matches!(style, GraphStyle::Line),
            "Plotting more than 2 series at a time is only supported for line style"
        );

        let start = line_set[0];
        let end = line_set[1];

        let filled = match style {
            GraphStyle::Auto => start <= end,
            GraphStyle::Filled => true,
            GraphStyle::Line => false,
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

        let chunks = iter.chunks_exact(M);
        let mut tip = chunks.remainder().to_vec();
        let mut row: Vec<[bool; M]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(M, false);
            row.push(tip.try_into().unwrap());
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_build_braille_row() {
        struct Foo;
        impl RowBuildable for Foo {}

        #[rustfmt::skip]
        let input_row = [
            vec![[ true, false]],
            vec![[false,  true]],
            vec![[ true,  true]],
            vec![[false, false]],
        ];

        #[rustfmt::skip]
        let expected = vec![
            [
                [ true, false],
                [false,  true],
                [ true,  true],
                [false, false],
            ],
        ];

        let actual = Foo::assemble_row(&input_row);

        assert_eq!(expected, actual);
    }
}
