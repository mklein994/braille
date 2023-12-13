mod columns;
mod lines;

pub use columns::Columns;
pub use lines::Lines;

use crate::GraphStyle;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    inner: u8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RawChar {
    inner: [[bool; 2]; 4],
}

impl From<Char> for RawChar {
    fn from(value: Char) -> Self {
        Self {
            inner: value.as_dot_pairs(),
        }
    }
}

impl From<RawChar> for Char {
    fn from(value: RawChar) -> Self {
        Char::new(value.inner)
    }
}

impl std::ops::Index<RowCol> for RawChar {
    type Output = bool;

    fn index(&self, index: RowCol) -> &Self::Output {
        &self.inner[index.row][index.col]
    }
}

impl std::ops::IndexMut<RowCol> for RawChar {
    fn index_mut(&mut self, index: RowCol) -> &mut Self::Output {
        &mut self.inner[index.row][index.col]
    }
}

impl Char {
    /// Turns an array of dot pairs into a braille character.
    ///
    /// # Example
    ///
    /// ```
    /// # use braille::graph::braille::Char;
    /// use braille::BrailleLines;
    /// use braille::graph::braille::Brailleish;
    /// assert_eq!(
    ///     Char::new([
    ///         [true, true],
    ///         [false, true],
    ///         [true, false],
    ///         [true, true],
    ///     ]).as_char(),
    ///     '⣝'
    /// );
    /// ```
    ///
    /// See also: <https://en.wikipedia.org/wiki/Braille_Patterns>
    #[must_use]
    pub fn new(dot_pairs: [[bool; 2]; 4]) -> Self {
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

        let mut dots = 0;

        for (index, bit) in bits.iter().enumerate() {
            if *bit {
                let position = u32::try_from(index).unwrap();
                dots += 2_u8.pow(position);
            }
        }

        Self { inner: dots }
    }

    #[must_use]
    pub fn as_char(self) -> char {
        char::from_u32(0x2800 + u32::from(self.inner)).expect("braille char not valid")
    }

    #[must_use]
    pub fn as_dot_pairs(&self) -> [[bool; 2]; 4] {
        let dots = self.inner;
        [
            [dots & 2_u8.pow(0) != 0, dots & 2_u8.pow(3) != 0],
            [dots & 2_u8.pow(1) != 0, dots & 2_u8.pow(4) != 0],
            [dots & 2_u8.pow(2) != 0, dots & 2_u8.pow(5) != 0],
            [dots & 2_u8.pow(6) != 0, dots & 2_u8.pow(7) != 0],
        ]
    }

    #[must_use]
    pub fn as_dot_quads(&self) -> [[bool; 4]; 2] {
        let dots = self.inner;
        [
            [
                dots & 2_u8.pow(0) != 0,
                dots & 2_u8.pow(1) != 0,
                dots & 2_u8.pow(2) != 0,
                dots & 2_u8.pow(6) != 0,
            ],
            [
                dots & 2_u8.pow(3) != 0,
                dots & 2_u8.pow(4) != 0,
                dots & 2_u8.pow(5) != 0,
                dots & 2_u8.pow(7) != 0,
            ],
        ]
    }
}

// impl std::ops::Index<(usize, usize)> for Char {
//     type Output = bool;
//
//     fn index(&self, index: (usize, usize)) -> &Self::Output {
//         match index {
//             (0, 0) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 1) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 2) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 3) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 0) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 1) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 2) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 3) => &(self.inner & 2_u8.pow(0) != 0),
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowCol {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: usize,
    y: usize,
}

// impl std::ops::Index<RowCol> for Char {
//     type Output = bool;
//
//     fn index(&self, index: RowCol) -> &Self::Output {
//         let RowCol { row, col } = index;
//         match (row, col) {
//             (0, 0) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 1) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 2) => &(self.inner & 2_u8.pow(0) != 0),
//             (0, 3) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 0) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 1) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 2) => &(self.inner & 2_u8.pow(0) != 0),
//             (1, 3) => &(self.inner & 2_u8.pow(0) != 0),
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }

// impl std::ops::Index<Coord> for Char {
//     type Output = bool;
//
//     fn index(&self, index: Coord) -> &Self::Output {
//         self.index(RowCol {
//             row: index.y,
//             col: index.x,
//         })
//     }
// }

impl From<[[bool; 2]; 4]> for Char {
    fn from(value: [[bool; 2]; 4]) -> Self {
        Self::new(value)
    }
}

impl TryFrom<char> for Char {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let num = u32::from(value);

        anyhow::ensure!(
            (0x2800..=0x28ff).contains(&num),
            "Char is not a valid braille character"
        );

        Ok(Self {
            inner: (num - 0x2800).try_into().unwrap(),
        })
    }
}

impl From<u8> for Char {
    fn from(value: u8) -> Self {
        Self { inner: value }
    }
}

impl std::ops::BitOr for Char {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner.bitor(rhs.inner),
        }
    }
}

impl std::ops::BitAnd for Char {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner.bitand(rhs.inner),
        }
    }
}

impl std::ops::BitXor for Char {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner.bitxor(rhs.inner),
        }
    }
}

impl std::ops::BitOrAssign for Char {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner);
    }
}

impl std::ops::BitAndAssign for Char {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner);
    }
}

impl std::ops::BitXorAssign for Char {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner);
    }
}

impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:08b}", self.inner)
        } else {
            self.as_char().fmt(f)
        }
    }
}

struct Chars {
    inner: Vec<Vec<Char>>,
}

impl std::fmt::Display for Chars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for column in row {
                write!(f, "{column}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub trait Brailleish<const DOTS_PER_VALUE: usize> {
    /// Turn a value into its representation of braille dots for that group
    ///
    /// # Example:
    ///
    /// Let's say the width is 4 characters wide, and the input is all integers in the range `[-3,
    /// 4]` incrementing by one. The braille pattern for a line graph 4 characters wide would look
    /// like this:
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
    /// # use braille::{BrailleLines, GraphStyle};
    /// use braille::graph::braille::Brailleish;
    /// //   ┌──── -2 (dot 2)
    /// //  -* **
    /// //      └─  0 (dot 4)
    /// assert_eq!(
    ///     vec![[false, true], [true, true]],
    ///     <BrailleLines as Brailleish<2>>::into_dot_groups(2, 4, GraphStyle::default())
    /// );
    /// ```
    ///
    /// In the second example row, the input value is 3, which translates to dot 7. The value is above
    /// zero, and looks like this:
    ///
    /// ```
    /// # use braille::{BrailleLines, GraphStyle};
    /// use braille::graph::braille::Brailleish;
    /// //          ┌── 3 (dot 7)
    /// // -- -* ** *-
    /// //     └─────── 0 (dot 4)
    /// assert_eq!(
    ///     vec![[false, false], [false, true], [true, true], [true, false]],
    ///     <BrailleLines as Brailleish<2>>::into_dot_groups(7, 4, GraphStyle::default())
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
    fn into_dot_groups(value: u16, zero: u16, style: GraphStyle) -> Vec<[bool; DOTS_PER_VALUE]> {
        let prefix_length = usize::from(value.min(zero) - 1);
        let mut iter = vec![false; prefix_length];

        let filled = match style {
            GraphStyle::Auto => value >= zero,
            GraphStyle::Filled => true,
            GraphStyle::Line => false,
        };

        let stem_length = usize::from(value.abs_diff(zero));
        for i in 0..=stem_length {
            if (value < zero && i == 0) || (value >= zero && i == stem_length) {
                iter.push(true);
            } else {
                iter.push(filled);
            }
        }

        let chunks = iter.chunks_exact(DOTS_PER_VALUE);
        let mut tip = chunks.remainder().to_vec();
        let mut groups: Vec<[bool; DOTS_PER_VALUE]> = chunks
            .into_iter()
            .map(|chunk| chunk.try_into().unwrap())
            .collect();
        if !tip.is_empty() {
            tip.resize(DOTS_PER_VALUE, false);
            groups.push(<[_; DOTS_PER_VALUE]>::try_from(tip).unwrap());
        }

        groups
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn scale(value: f64, minimum: f64, maximum: f64, min: u16, max: u16) -> u16 {
        assert!(
            value >= minimum && value <= maximum,
            "value out of bounds: {value} [{minimum}, {maximum}]"
        );
        let slope = f64::from(max - min) / (maximum - minimum);
        min + (slope * (value - minimum)).round() as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_list() {
        let input = "\
⠀⡈⡔⠉⠄⠀⠀⠐⠁⠀⠀⠠⠉⠑⣑⠊⠡⠀⠀⠀⠌⡀⠀⠀⠀⠀
⠀⠠⠀⠀⠈⠀⠀⡀⠈⠀⠠⠁⠀⠈⡠⡀⠀⢁⠀⠀⠀⠀⠀⠀⠀⠀
⠠⠄⠈⠀⠀⠁⠀⠀⠀⡀⠂⠀⠀⣀⠀⠂⠀⠀⡀⠈⠀⠠⠀⠀⠀⠀
⢂⠀⠀⠀⠀⠈⢀⠀⠀⠐⠀⠀⠀⡀⠀⢈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄
⡐⠀⠀⠂⠀⠀⠂⠀⠀⠂⠀⠀⠀⠀⠀⠀⠁⠀⠈⠂⠀⠀⠀⠀⠠⠀
⠀⠂⠀⠀⠀⠀⠠⠀⠐⠐⠀⠀⠘⠀⠀⠀⠈⡀⠀⠂⠀⠀⠁⠀⠄⠆
⠀⠐⠀⠠⠀⠀⠂⠄⠁⠀⠀⠀⠂⠀⠀⠀⠂⢀⠠⠠⠀⠀⠀⠠⠠⠀
⠀⠀⠂⠀⠀⠀⠀⣈⠀⠀⠄⠐⠄⠀⠀⠀⠀⠀⡀⠀⠄⠀⠐⠄⢄⠀
⠀⠀⠈⡀⠄⠐⠠⠀⠄⠀⠀⠂⠀⠀⠀⠀⠐⠀⠤⠀⠠⠀⡐⠠⠀⠀
⠀⠀⠀⠐⢄⡤⠁⠀⠐⣀⠜⡐⠀⠀⠀⠀⠀⢄⠀⠡⣀⠶⡠⢃⠂⠀";

        let char_list = input
            .lines()
            .map(|line| line.chars().map(|c| Char::try_from(c).unwrap()).collect())
            .collect();

        let chars = Chars { inner: char_list };

        insta::assert_display_snapshot!(chars);
    }
}
