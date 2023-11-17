use std::process::{Command, Stdio};

fn from_numbers(inputs: &[Option<f64>]) -> String {
    let lines = inputs
        .iter()
        .map(|x| x.map(|value| value.to_string()).unwrap_or_default())
        .collect::<Vec<_>>();
    lines.join("\n")
}

fn get_output_from_str<In, Iter, S>(input: In, args: Iter) -> (String, String)
where
    In: AsRef<std::ffi::OsStr>,
    Iter: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let bin = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/braille"); // bin name
    let echo = Command::new("echo")
        .arg(input)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new(bin)
        .args(args)
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    (stdout, stderr)
}

fn get_output<I, S>(inputs: &[Option<f64>], args: I) -> (String, String)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    get_output_from_str(from_numbers(inputs), args)
}

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
                [$min.to_string(), $max.to_string(), $width.to_string()],
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
                [$min.to_string(), $max.to_string(), $width.to_string()],
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
                [$min.to_string(), $max.to_string(), $width.to_string()],
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

t!("columns", column_characters_1_8_1, 1, 8, 1);

t!(
    "columns",
    columns_sine_wave,
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
    "bars",
    bars_sine_wave,
    -1,
    1,
    40,
    (0..100)
        .map(|x| Some(f64::sin(f64::from(x) / 10.)))
        .collect()
);

macro_rules! t_modeline {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            let (stdout, stderr) = get_output_from_str($input, ["-m".to_string()]);
            insta::assert_snapshot!(stdout);
            assert!(stderr.is_empty());
        }
    };
}

t_modeline!(
    test_correct_modeline,
    r"braille: -3 4 4
-2
0
1

3
4"
);
