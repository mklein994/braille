use braille::Opt;

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

    let mut lines = braille::get_lines();
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
            let transposed = braille::transpose_row(&buffer);
            let braille_char = braille::to_braille_char_row(&transposed);
            println!("{braille_char}");
        }

        buffer.fill(None);
    }

    Ok(())
}
