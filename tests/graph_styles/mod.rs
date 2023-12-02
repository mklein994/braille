use braille::{GraphKind, GraphStyle};

use crate::tt;

fn get_values() -> Vec<f64> {
    use std::f64::consts::PI;
    let mut values = vec![];
    let mut i = -8. * PI;
    while i < 8. * PI {
        values.push(i);
        i += 1.;
    }
    values
}

fn curve1() -> Vec<f64> {
    get_values().into_iter().map(|x| f64::cos(x / 5.)).collect()
}

fn curve2() -> Vec<f64> {
    get_values().into_iter().map(|x| f64::sin(x / 4.)).collect()
}

fn get_both_curves() -> Vec<(f64, f64)> {
    std::iter::zip(curve1(), curve2()).collect()
}

fn get_input(kind: GraphKind, style: GraphStyle, per: u8) -> String {
    use std::fmt::Write;

    assert!(
        matches!(
            (kind, style, per),
            (
                GraphKind::Bars | GraphKind::BrailleBars | GraphKind::BrailleColumns,
                _,
                1 | 2
            ) | (GraphKind::Columns, GraphStyle::Filled, 1)
        ),
        "invalid flag combo"
    );

    let kind_flag = match kind {
        GraphKind::Bars => "-B",
        GraphKind::Columns => "-C",
        GraphKind::BrailleBars => "-b",
        GraphKind::BrailleColumns => "-c",
    };

    let style_flag = match style {
        GraphStyle::Auto => "-sa",
        GraphStyle::Line => "-sl",
        GraphStyle::Filled => "-sf",
    };

    let per_flag = match per {
        1 => "",
        2 => "-p2",
        _ => unreachable!(),
    };

    let modeline = ["braille", kind_flag, style_flag, per_flag, "10"].join(" ");

    let values = if per == 1 {
        curve1()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    } else {
        get_both_curves()
            .into_iter()
            .map(|(left, right)| format!("{left}\t{right}"))
            .collect::<Vec<_>>()
    };

    let input = values
        .into_iter()
        .fold(format!("{modeline}\n"), |mut input, line| {
            let _ = writeln!(input, "{line}");
            input
        });

    input.trim_end().to_string()
}

#[test]
fn check_against_jq() {
    use std::process::{Command, Stdio};

    let rust_values: Vec<(f64, f64)> = get_both_curves();
    let jq_output = String::from_utf8(
        Command::new("jq")
            .arg("-nrf")
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/curve-2.jq"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let jq_values: Vec<(f64, f64)> = jq_output
        .lines()
        .map(|line| -> Result<_, std::num::ParseFloatError> {
            let (left, right) = line.split_once('\t').unwrap();
            Ok((left.parse()?, right.parse()?))
        })
        .collect::<Result<_, _>>()
        .unwrap();

    assert_eq!(rust_values, jq_values);
}

macro_rules! t {
    (#style_test $name:ident, $kind:expr, $per:literal) => {{
        tt!(#concat!(stringify!($name), "_", stringify!($per), "_auto"), get_input($kind, Auto, $per), ["-m"]);
        tt!(#concat!(stringify!($name), "_", stringify!($per), "_filled"), get_input($kind, Filled, $per), ["-m"]);
        tt!(#concat!(stringify!($name), "_", stringify!($per), "_line"), get_input($kind, Line, $per), ["-m"]);
    }};

    ($name:ident, $kind:expr) => {
        #[test]
        fn $name() {
            use GraphStyle::*;

            t!(#style_test $name, $kind, 1);
            t!(#style_test $name, $kind, 2);
        }
    };

    ($name:ident, $kind:expr, $style:expr, $per:expr) => {
        tt!($name, get_input($kind, $style, $per), ["-m"]);
    };
}

t!(test_columns, GraphKind::Columns, GraphStyle::default(), 1);

t!(test_bars, GraphKind::Bars);
t!(test_braille_columns, GraphKind::BrailleColumns);
t!(test_braille_bars, GraphKind::BrailleBars);
