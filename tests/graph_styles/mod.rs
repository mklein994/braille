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

fn get_input(kind: GraphKind, style: GraphStyle, per: u8) -> String {
    use std::fmt::Write;

    assert!(
        matches!(
            (kind, style, per),
            (GraphKind::BrailleBars | GraphKind::BrailleColumns, _, 1 | 2)
                | (GraphKind::Bars | GraphKind::Columns, GraphStyle::Filled, 1)
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

    let mut values = vec![];
    for x in get_values() {
        let mut line = vec![];
        if per >= 1 {
            line.push(f64::cos(x / 5.).to_string());
        }

        if per == 2 {
            line.push(f64::sin(x / 4.).to_string());
        }

        values.push(line.join("\t"));
    }

    let input = values
        .into_iter()
        .fold(format!("{modeline}\n"), |mut input, line| {
            let _ = writeln!(input, "{line}");
            input
        });

    input.trim_end().to_string()
}

macro_rules! t {
    ($name:ident, $kind:expr) => {
        #[test]
        fn $name() {
            use GraphStyle::*;

            tt!(#concat!(stringify!($name), "_1_auto"), get_input($kind, Auto, 1), ["-m"]);
            tt!(#concat!(stringify!($name), "_1_filled"), get_input($kind, Filled, 1), ["-m"]);
            tt!(#concat!(stringify!($name), "_1_line"), get_input($kind, Line, 1), ["-m"]);

            tt!(#concat!(stringify!($name), "_2_auto"), get_input($kind, Auto, 2), ["-m"]);
            tt!(#concat!(stringify!($name), "_2_filled"), get_input($kind, Filled, 2), ["-m"]);
            tt!(#concat!(stringify!($name), "_2_line"), get_input($kind, Line, 2), ["-m"]);
        }
    };

    ($name:ident, $kind:expr, $style:expr, $per:expr) => {
        tt!($name, get_input($kind, $style, $per), ["-m"]);
    };
}

t!(test_bars, GraphKind::Bars, GraphStyle::default(), 1);
t!(test_columns, GraphKind::Columns, GraphStyle::default(), 1);

t!(test_braille_columns, GraphKind::BrailleColumns);
t!(test_braille_bars, GraphKind::BrailleBars);

#[test]
fn check_against_jq() {
    use std::process::{Command, Stdio};

    let rust_values: Vec<f64> = get_values().iter().map(|x| f64::cos(*x / 5.)).collect();
    let jq_output = String::from_utf8(
        Command::new("jq")
            .arg("-nrf")
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/curve-1.jq"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let jq_values = jq_output
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<f64>, _>>()
        .unwrap();

    assert_eq!(rust_values, jq_values);
}
