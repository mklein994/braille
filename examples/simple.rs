use std::collections::HashSet as Set;
use std::f64::consts::PI;

use braillefb::Framebuffer;

fn scale(value: f64, i_min: f64, i_max: f64, f_min: f64, f_max: f64) -> f64 {
    debug_assert!(i_min < i_max);
    debug_assert!(f_min < f_max);
    debug_assert!(i_min <= value && value <= i_max);

    let slope = (f_max - f_min) / (i_max - i_min);
    f_min + (slope * (value - i_min))
}

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

fn main() -> anyhow::Result<()> {
    let (width, height) = get_terminal_size()?;
    // let width = 58;
    // let height = 29;
    let width = width * 2;
    let height = height * 4;
    // let mut dots = get_input(width, height)?;
    let mut dots = get_rose(width.into(), height.into());

    let mut bits: Vec<bool> = Vec::with_capacity(usize::from(width) * usize::from(height));
    for y in (0..height).rev() {
        for x in 0..width {
            bits.push(dots.remove(&Dot::new(x, y)));
        }
    }

    let fb = Framebuffer::new(&bits, width.into(), height.into());

    print!("{fb}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Dot {
    y: u16,
    x: u16,
}

impl Dot {
    fn new(x: u16, y: u16) -> Self {
        // Self { x, y }
        Self { y, x }
    }

    fn from_point(point: Point, width: f64, height: f64) -> Self {
        // let x = scale(point.x, -1., 1., 0., f64::from(width - 1)).round() as u16;
        let x = scale(point.x, -1., 1., 0., width - 1.).round() as u16;
        // let y = scale(point.y, -1., 1., 0., f64::from(height - 1)).round() as u16;
        let y = scale(point.y, -1., 1., 0., height - 1.).round() as u16;
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    y: f64,
    x: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        // Self { x, y }
        Self { y, x }
    }
}

fn rose(t: f64, k: f64, now: f64) -> Point {
    let (t_sin, t_cos) = t.sin_cos();
    let x = (k * t + now).sin() * t_cos;
    let y = (k * t).sin() * t_sin;
    Point::new(x, y)
}

fn get_rose(width: f64, height: f64) -> Set<Dot> {
    let mut dots = Set::new();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let k = 2. / 5.;
    let mut t = 0.;
    while t < 10. * PI {
        let point = rose(t, k, now * 0.25);

        let dot = Dot::from_point(point, width, height);

        dots.insert(dot);

        t += 1. / width.min(height);
    }

    dots
}

fn get_input(width: u16, height: u16) -> anyhow::Result<Set<Dot>> {
    let mut dots = Set::new();
    // let mut dots = Set::with_capacity((width * height).into());
    for line in std::io::stdin().lines() {
        let line = line?;
        let (left, right) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
        let point = Point::new(left.parse()?, right.parse()?);
        let dot = Dot::from_point(point, width.into(), height.into());

        dots.insert(dot);
    }

    Ok(dots)
}
