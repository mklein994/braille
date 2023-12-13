mod graph_styles;
mod modeline;
mod util;

use util::*;

#[macro_export]
macro_rules! tt {
    (#$snapshot_name:expr, $stdout:ident, $stderr:ident) => {{
        insta::assert_snapshot!($snapshot_name, $stdout);
        assert!($stderr.is_empty(), "stderr is not empty:\n{}", $stderr);
    }};

    (#$stdout:ident, $stderr:ident) => {{
        insta::assert_snapshot!($stdout);
        insta::assert_snapshot!($stderr);
    }};

    (#$args:expr) => {
        let (stdout, stderr) = $crate::util::get_output($args);
        tt!(#stdout, stderr);
    };

    ($name:ident, $args:expr) => {
        #[test]
        fn $name() {
            tt!(#$args);
        }
    };

    (#$input:expr, $args:expr) => {
        let (stdout, stderr) = $crate::util::get_output_from_str($input, $args);
        tt!(#stdout, stderr);
    };

    (#$snapshot_name:expr, $input:expr, $args:expr) => {
        let (stdout, stderr) = $crate::util::get_output_from_str($input, $args);
        tt!(#$snapshot_name, stdout, stderr);
    };

    ($name:ident, $input:expr, $args:expr) => {
        #[test]
        fn $name() {
            tt!(#$input, $args);
        }
    };
}

macro_rules! t {
    (auto $name:ident, $width:literal, $gen:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = $gen.into_iter().map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output_from_numbers(&input, [$width.to_string()]);
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };

    ($name:ident, $min:expr, $max:expr, $width:expr) => {
        #[test]
        fn $name() {
            let input: Vec<_> = (($min)..=($max)).map(|x| Some(x as f64)).collect();
            let (stdout, stderr) = get_output_from_numbers(
                &input,
                [
                    "-r".to_string(),
                    format!("{}:{}", $min, $max),
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
            let (stdout, stderr) = get_output_from_numbers(
                &input,
                [
                    "-r".to_string(),
                    format!("{}:{}", $min, $max),
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
            let (stdout, stderr) = get_output_from_numbers(
                &input,
                [
                    "-r".to_string(),
                    format!("{}:{}", $min, $max),
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
            let (stdout, stderr) = get_output_from_numbers(
                &input,
                [
                    "--kind".to_string(),
                    $kind.to_string(),
                    "-r".to_string(),
                    format!("{}:{}", $min, $max),
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
            let (stdout, stderr) = get_output_from_numbers(
                &input,
                [
                    "--kind".to_string(),
                    $kind.to_string(),
                    "-r".to_string(),
                    format!("{}:{}", $min, $max),
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

#[test]
fn thick_sine_wave() {
    let input = r"4 6
5 7
3 5
1 3
0 2
2 4
4 6
5 7
3 5
1 3
0 2
1 3
3 5
5 7
4 6
2 4";

    let (stdout, stderr) = util::get_output_from_str(input, ["--per", "2", "4"]);

    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn hollow_sine_wave() {
    let input = r"6 4
7 5
5 3
3 1
2 0
4 2
6 4
7 5
5 3
3 1
2 0
3 1
5 3
7 5
6 4
4 2";

    let (stdout, stderr) = util::get_output_from_str(input, ["--per", "2", "-s", "line", "4"]);

    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn thick_staircase() {
    let input = r"0 2
1 3
2 4
3 5
4 6
5 7
6 8
7 9
8 10
9 11
10 12
11 13
12 14
13 15
14 16
15 17";

    let (stdout, stderr) = util::get_output_from_str(input, ["--per", "2", "4"]);

    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn braille_column_read_pairs_from_file() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sine_area.tsv");

    let (stdout, stderr) = util::get_output(["-f", path, "-c", "--per", "2", "2"]);

    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn braille_line_line_style_all_negative() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/single_all_negative.tsv");

    let (stdout, stderr) = util::get_output(["-f", path, "-s", "line", "56"]);

    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn invalid_input_single() {
    let input = "hello";
    let (stdout, stderr) = util::get_output_from_str::<_, &[&str], _>(input, &[]);
    insta::assert_snapshot!(stderr);
    assert!(stdout.is_empty());
}

#[test]
fn invalid_input_multiple() {
    let input = "1 hello";
    let (stdout, stderr) = util::get_output_from_str::<_, &[&str], _>(input, &["-p", "2"]);
    insta::assert_snapshot!(stderr);
    assert!(stdout.is_empty());
}

#[test]
fn braille_columns_odd_number_of_values() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/large_value_at_end.txt");

    let (stdout, stderr) = util::get_output(["-f", path, "-c", "10"]);
    insta::assert_snapshot!(stdout);
    assert!(stderr.is_empty());
}

#[test]
fn braille_3_values_per_line() {
    let (stdout, stderr) = util::get_output_from_str(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/braille_series_3_modeline.txt"
        )),
        ["-m"],
    );
    insta::assert_snapshot!(stdout);
    insta::assert_snapshot!(stderr);
}
