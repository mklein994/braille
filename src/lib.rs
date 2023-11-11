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

impl Opt {
    pub fn from_args() -> anyhow::Result<Self> {
        let mut args = std::env::args().skip(1);
        let minimum = args.next().expect("height missing").parse()?;
        let maximum = args.next().expect("max missing").parse()?;
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
pub fn to_braille_char_row(transposed: &[[[bool; 2]; 4]]) -> String {
    let mut line = String::new();
    for character in transposed {
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
        //
        let bits = [
            character[0][0],
            character[1][0],
            character[2][0],
            character[0][1],
            character[1][1],
            character[2][1],
            character[3][0],
            character[3][1],
        ];

        let mut block = 0x2800_u32;

        for set_position in bits
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x { Some(i) } else { None })
        {
            block += (2_u32).pow(u32::try_from(set_position).unwrap());
        }

        line.push(char::from_u32(block).unwrap());
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

        if !(column == longest - 1 && braille_character.iter().flatten().all(|x| x == &false)) {
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
