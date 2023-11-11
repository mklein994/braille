use std::process::{Command, Stdio};

fn get_output<I, S>(inputs: &[Option<f64>], args: I) -> (String, String)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let bin = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/braille"); // bin name
    let input = inputs
        .iter()
        .map(|x: &Option<f64>| x.map(|value| value.to_string()).unwrap_or_default())
        .collect::<Vec<_>>();
    let echo = Command::new("echo")
        .arg(input.join("\n"))
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

macro_rules! t {
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
