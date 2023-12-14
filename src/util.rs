/// Determine the terminal size from the terminal itself if possible, with fallbacks
pub fn get_terminal_size() -> anyhow::Result<(u16, u16)> {
    use terminal_size::{Height, Width};

    if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
        Ok((width, height))
    } else {
        use std::env::VarError;

        let parse_from_environment = |name, fallback| match std::env::var(name) {
            Ok(value) => Ok(value.parse()?),
            Err(VarError::NotPresent) => Ok(fallback),
            Err(err) => Err(anyhow::Error::from(err)),
        };

        let width = parse_from_environment("COLUMNS", 80)?;
        let height = parse_from_environment("LINES", 24)?;
        Ok((width, height))
    }
}

/// Scale a value from one range into another
#[must_use]
pub fn scale(value: f64, i_min: f64, i_max: f64, f_min: f64, f_max: f64) -> f64 {
    let slope = (f_max - f_min) / (i_max - i_min);
    f_min + (slope * (value - i_min))
}
