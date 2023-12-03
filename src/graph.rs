pub mod blocks;
pub mod braille;
pub mod mini_blocks;

use std::io::{LineWriter, Write};
use std::str::FromStr;

use crate::opt::{Config, Configurable, GraphStyle, ValueIter};
use crate::InputLine;

pub trait Graphable<T, Conf: Configurable = Config>
where
    InputLine<T>: FromStr,
{
    fn new(config: Conf) -> Self;

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
    ) -> anyhow::Result<()>
    where
        InputLine<T>: FromStr;
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

pub trait Transposable {
    /// Turn a row of dot sets into a list of braille dot blocks
    #[must_use]
    fn transpose_row<const N: usize, const M: usize>(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_transpose_braille() {
        struct Foo;
        impl Transposable for Foo {}

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

        let actual = Foo::transpose_row(&input_row);

        assert_eq!(expected, actual);
    }
}
