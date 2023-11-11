use braille::Opt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args()?;

    let min = 1;
    let max = opt.width * 2;
    let slope = (f64::from(max) - f64::from(min)) / (opt.maximum - opt.minimum);
    let scale = |value: f64| {
        assert!(
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
    let mut buffer = [vec![], vec![], vec![], vec![]];
    let mut has_more_lines = true;
    while has_more_lines {
        for buffer_line in &mut buffer {
            let line = lines.next();
            if line.is_none() {
                has_more_lines = false;
            }

            if let Some(new_line) = line
                .transpose()?
                .flatten()
                .map(scale)
                .map(|value| braille::into_bit_pairs(value, zero))
            {
                *buffer_line = new_line;
            }
        }

        if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
            let transposed = braille::transpose_row(&buffer);
            let braille_char = braille::to_braille_char_row(&transposed);
            println!("{braille_char}");
        }

        buffer.fill(vec![]);
    }

    Ok(())
}
