use std::process::{Command, Stdio};

pub fn from_numbers(inputs: &[Option<f64>]) -> String {
    let lines = inputs
        .iter()
        .map(|x| x.map(|value| value.to_string()).unwrap_or_default())
        .collect::<Vec<_>>();
    lines.join("\n")
}

pub fn get_output_from_str<In, Iter, S>(input: In, args: Iter) -> (String, String)
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

pub fn get_output<I, S>(inputs: &[Option<f64>], args: I) -> (String, String)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    get_output_from_str(from_numbers(inputs), args)
}
