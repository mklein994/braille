mod columns;
mod lines;

pub use columns::Columns;
pub use lines::Lines;

use crate::GraphStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BrailleChar {
    inner: u8,
}

impl BrailleChar {
    /// Turns an array of dot pairs into a braille character.
    ///
    /// # Example
    ///
    /// ```
    /// use braille::BrailleLines;
    /// use braille::graph::braille::BrailleLike;
    /// assert_eq!(
    ///     braille::BrailleChar::new([
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
    pub fn as_dot_pairs(self) -> [[bool; 2]; 4] {
        let dots = self.inner;
        [
            [dots & 2_u8.pow(0) != 0, dots & 2_u8.pow(3) != 0],
            [dots & 2_u8.pow(1) != 0, dots & 2_u8.pow(4) != 0],
            [dots & 2_u8.pow(2) != 0, dots & 2_u8.pow(5) != 0],
            [dots & 2_u8.pow(6) != 0, dots & 2_u8.pow(7) != 0],
        ]
    }
}

impl std::fmt::Display for BrailleChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_char().fmt(f)
    }
}

pub trait BrailleLike<const DOTS_PER_VALUE: usize> {
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
    /// use braille::graph::braille::BrailleLike;
    /// //   ┌──── -2 (dot 2)
    /// //  -* **
    /// //      └─  0 (dot 4)
    /// assert_eq!(
    ///     vec![[false, true], [true, true]],
    ///     <BrailleLines as BrailleLike<2>>::into_dot_groups(2, 4, GraphStyle::default())
    /// );
    /// ```
    ///
    /// In the second example row, the input value is 3, which translates to dot 7. The value is above
    /// zero, and looks like this:
    ///
    /// ```
    /// # use braille::{BrailleLines, GraphStyle};
    /// use braille::graph::braille::BrailleLike;
    /// //          ┌── 3 (dot 7)
    /// // -- -* ** *-
    /// //     └─────── 0 (dot 4)
    /// assert_eq!(
    ///     vec![[false, false], [false, true], [true, true], [true, false]],
    ///     <BrailleLines as BrailleLike<2>>::into_dot_groups(7, 4, GraphStyle::default())
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

        let filled = match (value, zero, style) {
            (v, z, GraphStyle::Auto) => v >= z,
            (_, _, GraphStyle::Filled) => true,
            (_, _, GraphStyle::Line) => false,
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
            groups.push(tip.try_into().unwrap());
        }

        groups
    }
}
