pub fn get_lines() -> impl Iterator<Item = Result<Option<f64>, impl std::error::Error>> {
    std::io::stdin().lines().map_while(Result::ok).map(|x| {
        if x.is_empty() {
            Ok(None)
        } else {
            Some(x.parse()).transpose()
        }
    })
}

#[derive(Debug)]
pub struct Opt {
    pub minimum: f64,
    pub maximum: f64,
    pub width: u32,
}

fn parse_first_arg(arg: &Option<String>) -> anyhow::Result<f64> {
    match arg.as_deref() {
        Some("-h" | "--help") | None => {
            anyhow::bail!("Usage: <STDIN> | braille <minimum> <maximum> [<width>]")
        }
        Some(min) => min
            .parse()
            .map_err(|err: std::num::ParseFloatError| anyhow::anyhow!(err)),
    }
}

impl Opt {
    pub fn from_args() -> anyhow::Result<Self> {
        let mut args = std::env::args().skip(1);

        let minimum = parse_first_arg(&args.next())?;
        let maximum = args
            .next()
            .map(|x| x.parse())
            .transpose()?
            .ok_or_else(|| anyhow::anyhow!("missing maximum argument"))?;
        let width = args
            .next()
            .map(|x| x.parse())
            .transpose()?
            .unwrap_or_else(|| {
                if let Some((terminal_size::Width(width), _)) = terminal_size::terminal_size() {
                    u32::from(width)
                } else {
                    80
                }
            });

        assert!(minimum < maximum);

        Ok(Self {
            minimum,
            maximum,
            width,
        })
    }
}

#[must_use]
pub fn into_bit_pairs(value: u32, zero: u32) -> Vec<[bool; 2]> {
    let mut iter = vec![false; usize::try_from(value.min(zero)).unwrap() - 1];
    iter.resize(
        iter.len() + usize::try_from(value.abs_diff(zero) + 1).unwrap(),
        true,
    );
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
    fn test_transpose_row() {
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
    fn test_transpose_row_2() {
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
