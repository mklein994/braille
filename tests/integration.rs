mod modeline;
mod util;

use util::*;

macro_rules! t {
    (auto $name:ident, $width:literal, $gen:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = $gen.into_iter().map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output(&input, [$width.to_string()]);
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($name:ident, $min:expr, $max:expr, $width:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = (($min)..=($max)).map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output(
                &input,
                [
                    "-r".to_string(),
                    $min.to_string(),
                    $max.to_string(),
                    $width.to_string(),
                ],
            );
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($name:ident, $min:expr, $max:expr, $width:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = (($min)..=($max)).map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output(
                &input,
                [
                    "-r".to_string(),
                    $min.to_string(),
                    $max.to_string(),
                    $width.to_string(),
                ],
            );
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($name:ident, $min:expr, $max:expr, $width:expr, $gen:expr) => {
        #[test]
        fn $name() {
            let input: ::std::vec::Vec<_> = $gen;
            let (stdout, stderr) = get_output(
                &input,
                [
                    "-r".to_string(),
                    $min.to_string(),
                    $max.to_string(),
                    $width.to_string(),
                ],
            );
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($kind:literal, $name:ident, $min:expr, $max:expr, $size:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = (($min)..=($max)).map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output(
                &input,
                [
                    "--kind".to_string(),
                    $kind.to_string(),
                    "-r".to_string(),
                    $min.to_string(),
                    $max.to_string(),
                    $size.to_string(),
                ],
            );
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($kind:literal, $name:ident, $min:expr, $max:expr, $size:expr, $gen:expr) => {
        #[test]
        fn $name() {
            let input: ::std::vec::Vec<_> = $gen;
            let (stdout, stderr) = get_output(
                &input,
                [
                    "--kind".to_string(),
                    $kind.to_string(),
                    "-r".to_string(),
                    $min.to_string(),
                    $max.to_string(),
                    $size.to_string(),
                ],
            );
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };
}

t!(width_5, -4, 3, 4);

t!(width_4_n8_n1, -8, -1, 4);
t!(width_4_n7_0, -7, 0, 4);
t!(width_4_n6_1, -6, 1, 4);
t!(width_4_n5_2, -5, 2, 4);
t!(width_4_n4_3, -4, 3, 4);
t!(width_4_n3_4, -3, 4, 4);
t!(width_4_n2_5, -2, 5, 4);
t!(width_4_n1_6, -1, 6, 4);
t!(width_4_0_7, 0, 7, 4);
t!(width_4_1_8, 1, 8, 4);

t!(
    sine_wave,
    -1,
    1,
    40,
    (0..100)
        .map(|x| Some(f64::sin(f64::from(x) / 10.)))
        .collect()
);

t!(
    contains_nulls,
    -3,
    4,
    4,
    ((-3)..=4)
        .map(|x| {
            if x == 2 {
                None
            } else {
                Some(f64::from(x))
            }
        })
        .collect()
);

t!(
    entire_screen_line_is_null,
    -6,
    20,
    5,
    ((-6)..=20)
        .map(|x| match x {
            5..=9 => None,
            _ => Some(f64::from(x)),
        })
        .collect()
);

t!(
    entire_screen_line_is_null_at_end_n4_11_4,
    -4,
    11,
    4,
    ((-4)..=11)
        .map(|x| match x {
            4..=11 => None,
            _ => Some(f64::from(x)),
        })
        .collect()
);

t!("bars", bar_characters_1_8_1, 1, 8, 1);

t!(
    "bars",
    bar_sine_wave,
    -1,
    1,
    40,
    (0..100)
        .map(|x| Some(f64::sin(f64::from(x) / 10.)))
        .collect()
);

t!(
    auto
    without_bounds_width_4,
    4,
    ((-3)..=4).collect::<Vec<_>>()
);

t!(
    "columns",
    column_sine_wave,
    -1,
    1,
    40,
    (0..100)
        .map(|x| Some(f64::sin(f64::from(x) / 10.)))
        .collect()
);

t!(
    "braille-columns",
    braille_columns_sine_wave,
    -1,
    1,
    40,
    (0..200)
        .map(|x| Some(f64::sin(f64::from(x) / 10.)))
        .collect()
);
