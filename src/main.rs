fn get_lines() -> impl Iterator<Item = Result<Option<f64>, impl std::error::Error>> {
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
    fn from_args() -> anyhow::Result<Self> {
        let mut args = std::env::args().skip(1);
        let minimum = args.next().expect("height missing").parse()?;
        let maximum = args.next().expect("max missing").parse()?;
        let width = args
            .next()
            .map(|x| x.parse())
            .transpose()?
            .unwrap_or_else(|| {
                if let Some((width, _)) = terminal_size::terminal_size() {
                    u32::from(width.0)
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

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args()?;

    let min = 1;
    let max = opt.width * 2;
    let slope = (f64::from(max) - f64::from(min)) / (opt.maximum - opt.minimum);
    let scale = |value: f64| {
        debug_assert!(
            value >= opt.minimum && value <= opt.maximum,
            "value out of bounds: {value} [{}, {}]",
            opt.minimum,
            opt.maximum
        );
        min + (slope * (value - opt.minimum)).round() as u32
    };

    let zero = if opt.minimum > 0. {
        min
    } else if opt.maximum < 0. {
        max
    } else {
        scale(0.)
    };

    let mut lines = get_lines();
    // let mut buffer = Vec::with_capacity(4);
    let mut buffer = [None, None, None, None];
    let mut has_more_lines = true;
    while has_more_lines {
        for buffer_line in &mut buffer {
            let line = lines.next();
            if line.is_none() {
                has_more_lines = false;
            }

            *buffer_line = line
                .transpose()?
                .unwrap_or_default()
                .map(scale)
                .map(|value| {
                    let mut iter = vec![false; usize::try_from(value.min(zero)).unwrap() - 1];
                    iter.resize(
                        iter.len() + usize::try_from(value.abs_diff(zero) + 1).unwrap(),
                        true,
                    );
                    let chunks = iter.chunks_exact(2);
                    let remainder: [bool; 2] = if chunks.remainder().is_empty() {
                        [false, false]
                    } else {
                        let mut rem = [false; 2];
                        for (i, r) in chunks.remainder().iter().enumerate() {
                            rem[i] = *r;
                        }
                        rem
                    };
                    let mut row: Vec<[bool; 2]> = chunks
                        .into_iter()
                        .map(|chunk| [chunk[0], chunk[1]])
                        .collect::<Vec<[bool; 2]>>();
                    row.push(remainder);
                    row
                });
        }

        if has_more_lines || buffer.iter().any(Option::is_some) {
            let transposed = transpose_row(&buffer);
            let braille_char = to_braille_char_row(&transposed);
            println!("{braille_char}");
        }

        buffer.fill(None);
    }

    Ok(())
}

fn to_braille_char_row(transposed: &[[[bool; 2]; 4]]) -> String {
    let mut line: Vec<char> = vec![];
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
    line.into_iter().collect()
}

fn transpose_row(input_row: &[Option<Vec<[bool; 2]>>; 4]) -> Vec<[[bool; 2]; 4]> {
    let longest = input_row
        .iter()
        .filter_map(|x| x.as_ref().map(Vec::len))
        .max()
        .unwrap();

    let mut output_row: Vec<[[bool; 2]; 4]> = vec![];
    for column in 0..longest {
        let mut braille_character = [[false, false]; 4];

        for (row_index, row) in input_row.iter().enumerate() {
            if let Some(row_column) = row.as_ref().and_then(|r| r.get(column)) {
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
            Some(vec![[ true,  true], [ true, true], [true, false]]),
            Some(vec![[false,  true], [ true, true], [true, false]]),
            Some(vec![[false, false], [ true, true], [true, false]]),
            Some(vec![[false, false], [false, true], [true, false]]),
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
            Some(vec![[false, false], [false, false], [true, false]               ]),
            Some(vec![[false, false], [false, false], [true,  true]               ]),
            Some(vec![[false, false], [false, false], [true,  true], [true, false]]),
            Some(vec![[false, false], [false, false], [true,  true], [true,  true]]),
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
