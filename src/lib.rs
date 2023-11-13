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

pub fn get_lines() -> impl Iterator<Item = Result<Option<f64>, std::num::ParseFloatError>> {
    std::io::stdin().lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    })
}

pub fn print_braille_lines(
    opt: &Opt,
    mut input_lines: impl Iterator<Item = Result<Option<f64>, std::num::ParseFloatError>>,
) -> anyhow::Result<()> {
    let min = 1; // reserve an empty line for null values
    let max = opt.width * 2; // braille characters are 2 dots wide
    let slope = f64::from(max - min) / (opt.maximum - opt.minimum);
    let scale = |value: f64| {
        assert!(
            value >= opt.minimum && value <= opt.maximum,
            "value out of bounds: {value} [{}, {}]",
            opt.minimum,
            opt.maximum
        );
        min + (slope * (value - opt.minimum)).round() as u16
    };

    // Clamp where 0 would fit to be inside the output range
    let zero = if opt.minimum > 0. {
        min
    } else if opt.maximum < 0. {
        max
    } else {
        scale(0.)
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
                .flatten()
                .map(scale)
                .map(|value| into_bit_pairs(value, zero))
            {
                *buffer_line = new_line;
            }
        }

        if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
            let transposed = transpose_row(&buffer);
            let braille_char = to_braille_char_row(&transposed);
            println!("{braille_char}");
        }

        buffer.fill(vec![]);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Opt {
    pub minimum: f64,
    pub maximum: f64,
    pub width: u16,
}

impl Opt {
    pub fn from_args(args: Vec<String>) -> anyhow::Result<Self> {
        let mut args = args.into_iter();

        let minimum = args.next().and_then(|x| x.parse().ok()).ok_or_else(|| {
            anyhow::anyhow!("Usage: <STDIN> | braille <minimum> <maximum> [<width>]")
        })?;
        let maximum = args
            .next()
            .map(|x| x.parse::<f64>())
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("missing maximum argument"))?;
        let width = args
            .next()
            .map(|x| x.parse())
            .transpose()?
            .or_else(|| {
                terminal_size::terminal_size().map(|(terminal_size::Width(width), _)| width)
            })
            .unwrap_or(80);

        assert!(minimum < maximum);

        Ok(Self {
            minimum,
            maximum,
            width,
        })
    }
}

#[must_use]
pub fn into_bit_pairs(value: u16, zero: u16) -> Vec<[bool; 2]> {
    let mut iter = vec![false; usize::from(value.min(zero)) - 1];
    iter.resize(iter.len() + usize::from(value.abs_diff(zero) + 1), true);
    let chunks = iter.chunks_exact(2);
    let remainder = {
        let mut rem = [false; 2];
        for (i, r) in chunks.remainder().iter().enumerate() {
            rem[i] = *r;
        }
        rem
    };
    let mut row: Vec<[bool; 2]> = chunks
        .into_iter()
        .map(|chunk| [chunk[0], chunk[1]])
        .collect();
    row.push(remainder);
    row
}

#[must_use]
pub fn to_braille_char(bit_pairs: [[bool; 2]; 4]) -> char {
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
        bit_pairs[0][0],
        bit_pairs[1][0],
        bit_pairs[2][0],
        bit_pairs[0][1],
        bit_pairs[1][1],
        bit_pairs[2][1],
        bit_pairs[3][0],
        bit_pairs[3][1],
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

#[must_use]
pub fn to_braille_char_row(transposed: &[[[bool; 2]; 4]]) -> String {
    let mut line = String::new();
    for bit_pairs in transposed {
        let braille_char = to_braille_char(*bit_pairs);

        line.push(braille_char);
    }
    line
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
        let actual = transpose_row(&input);
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

        assert_eq!(expected, transpose_row(&input));
    }
}
