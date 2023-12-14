use crate::util;
use braillefb::Framebuffer;
use std::collections::HashMap;
use std::io::{LineWriter, Write};

struct CartesianBound {
    min: f64,
    max: f64,
}

impl CartesianBound {
    fn new(min: f64, max: f64) -> Self {
        assert!(min < max);
        Self { min, max }
    }
}

impl Default for CartesianBound {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
        }
    }
}

struct CartesianBounds {
    x: CartesianBound,
    y: CartesianBound,
}

#[derive(Default, Clone, Copy)]
struct CartesianBoundsBuilder {
    x_min: Option<f64>,
    x_max: Option<f64>,
    y_min: Option<f64>,
    y_max: Option<f64>,
}

impl CartesianBoundsBuilder {
    fn x_min(&mut self, min: f64) {
        self.x_min = Some(min);
    }

    fn x_max(&mut self, max: f64) {
        self.x_max = Some(max);
    }

    fn y_min(&mut self, min: f64) {
        self.y_min = Some(min);
    }

    fn y_max(&mut self, max: f64) {
        self.y_max = Some(max);
    }

    // TODO: refactor this into structs and stuff
    fn build_with_list(self, list: &[Point]) -> CartesianBounds {
        let mut x_min = self
            .x_min
            .map_or((f64::MAX, false, false), |n| (n, true, false));
        let mut x_max = self
            .x_max
            .map_or((f64::MIN, false, true), |n| (n, true, true));
        let mut y_min = self
            .y_min
            .map_or((f64::MAX, false, false), |n| (n, true, false));
        let mut y_max = self
            .y_max
            .map_or((f64::MIN, false, true), |n| (n, true, true));

        let maybe_update = |old: &mut (f64, bool, bool), new: f64| {
            if !old.1 {
                if old.2 {
                    old.0 = old.0.max(new);
                } else {
                    old.0 = old.0.min(new);
                }
            }
        };

        for Point { x, y } in list {
            maybe_update(&mut x_min, *x);
            maybe_update(&mut x_max, *x);
            maybe_update(&mut y_min, *y);
            maybe_update(&mut y_max, *y);
        }

        CartesianBounds {
            x: CartesianBound::new(x_min.0, x_max.0),
            y: CartesianBound::new(y_min.0, y_max.0),
        }
    }
}

impl CartesianBounds {
    fn builder() -> CartesianBoundsBuilder {
        CartesianBoundsBuilder::default()
    }

    fn new(list: &[Point]) -> Self {
        let CartesianBound {
            min: mut x_min,
            max: mut x_max,
        } = CartesianBound::default();
        let CartesianBound {
            min: mut y_min,
            max: mut y_max,
        } = CartesianBound::default();
        for Point { x, y } in list {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
            y_min = y_min.min(*y);
            y_max = y_max.max(*y);
        }

        assert!(x_min < x_max);
        assert!(y_min < y_max);

        CartesianBounds {
            x: CartesianBound {
                min: x_min,
                max: x_max,
            },
            y: CartesianBound {
                min: y_min,
                max: y_max,
            },
        }
    }
}

struct CartesianCoords {
    bounds: CartesianBounds,
    inner: Vec<Point>,
}

impl CartesianCoords {
    fn new_with_bounds(list: Vec<Point>, bounds: CartesianBounds) -> Self {
        Self {
            inner: list,
            bounds,
        }
    }

    fn new(list: Vec<Point>) -> Self {
        let bounds = CartesianBounds::new(&list);
        Self {
            inner: list,
            bounds,
        }
    }
}

impl FromIterator<Point> for CartesianCoords {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let list = iter.into_iter().collect::<Vec<_>>();
        Self {
            bounds: CartesianBounds::new(&list),
            inner: list,
        }
    }
}

impl<'a> IntoIterator for &'a CartesianCoords {
    type Item = &'a Point;

    type IntoIter = std::slice::Iter<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

struct GridCoords {
    width: usize,
    height: usize,
    inner: HashMap<Pixel, bool>,
}

impl GridCoords {
    pub fn new(width: usize, height: usize) -> Self {
        let mut inner = HashMap::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let pixel = Pixel::new(x, y);
                inner.insert(pixel, false);
            }
        }

        debug_assert_eq!(width * height, inner.len());

        Self {
            width,
            height,
            inner,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Pixel {
    x: usize,
    y: usize,
}

impl Pixel {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl GridCoords {
    fn set_points(&mut self, points: &CartesianCoords) {
        for point in points {
            #[rustfmt::skip]
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
            let px = util::scale(point.x, points.bounds.x.min, points.bounds.x.max, 0., (self.width - 1) as f64).round() as usize;

            #[rustfmt::skip]
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
            let py = util::scale(point.y, points.bounds.y.min, points.bounds.y.max, 0., (self.height - 1) as f64).round() as usize;

            let point = Pixel::new(px, py);
            self.inner.entry(point).and_modify(|e| *e = true);
        }
    }
}

impl GridCoords {
    pub fn into_dots(self) -> Vec<bool> {
        let mut dots = Vec::with_capacity(self.width * self.height);
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = Pixel::new(x, y);
                dots.push(self.inner.get(&pixel).copied().unwrap_or_default());
            }
        }

        dots
    }
}

pub fn print_graph<W: Write>(opt: crate::Opt, mut writer: LineWriter<W>) -> anyhow::Result<()> {
    let grid = opt.grid.unwrap();
    let (width, height) = if grid.is_empty() {
        let (width, height) = crate::util::get_terminal_size()
            .map(|(w, h)| (usize::from(w) * 2, usize::from(h) * 4))?;
        let square = width.min(height);
        (square, square)
    } else if grid.len() == 1 {
        (grid[0], grid[0])
    } else {
        (grid[0], grid[1])
    };

    let (width, height) = (width, height);

    let mut coords: Vec<Point> = vec![];
    for line in std::io::stdin().lines() {
        let line = line?;
        let (x, y) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
        coords.push(Point::new(x.parse()?, y.parse()?));
    }

    let coords = if opt.grid_bounds.or(opt.x_bounds).or(opt.y_bounds).is_some() {
        let mut builder = CartesianBounds::builder();

        let (x_min, x_max) = opt
            .x_bounds
            .or(opt.grid_bounds)
            .map_or((None, None), |b| (b.min(), b.max()));
        let (y_min, y_max) = opt
            .y_bounds
            .or(opt.grid_bounds)
            .map_or((None, None), |b| (b.min(), b.max()));

        if let Some(x_min) = x_min {
            builder.x_min(x_min);
        }
        if let Some(x_max) = x_max {
            builder.x_max(x_max);
        }
        if let Some(y_min) = y_min {
            builder.y_min(y_min);
        }
        if let Some(y_max) = y_max {
            builder.y_max(y_max);
        }

        let bounds = builder.build_with_list(&coords);
        CartesianCoords::new_with_bounds(coords, bounds)
    } else {
        CartesianCoords::new(coords)
    };
    let mut grid = GridCoords::new(width, height);
    grid.set_points(&coords);

    let dots = grid.into_dots();
    let fb = Framebuffer::new(&dots, width, height);

    write!(writer, "{fb}")?;

    // let mut buf: Vec<bool> = vec![false; width * 2 * height * 4];

    // let bits: Vec<bool> = std::io::stdin()
    //     .lines()
    //     .map(|line| {
    //         line.map_err(|err| anyhow::anyhow!(err)).and_then(|l| {
    //             let (x, y) = l.split_once()
    //             let foo = l.split_once(char::is_ascii_whitespace)
    //                 .map(|(x, y)| [x.parse(), y.parse()]).unwrap();
    //         })
    //     })
    //     .collect::<Result<_, _>>()?;

    // let bits: Vec<bool> = vec![];
    //
    // let fb = Framebuffer::new(&bits, width * 2, height * 4);
    // writeln!(writer, "{fb}")?;

    Ok(())
    // if let Some(GridCommands::Coord {
    //     xmin,
    //     xmax,
    //     ymin,
    //     ymax,
    //     width,
    //     height,
    // }) = opt.commands
    // {
    //     let bits: Vec<bool> = std::io::stdin()
    //         .lines()
    //         .map(|line| {
    //             line.map_err(|err| anyhow::anyhow!(err))
    //                 .and_then(|l| l.parse::<bool>().map_err(|err| anyhow::anyhow!(err)))
    //         })
    //         .collect::<Result<_, _>>()?;
    //
    //     let (width, height) = if let (Some(width), Some(height)) = (width, height) {
    //         (width, height)
    //     } else {
    //         opt::get_terminal_size().map(|(w, h)| (w.into(), h.into()))?
    //     };
    //
    //     let width = width * 2;
    //     let height = height * 4;
    //
    //     writeln!(writer, "{}", BrailleGrid::new(&bits, width, height))?;
    //
    //     Ok(())
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn check_coord_stuff() {
        // -- -- -- -*
        // -- -- -- *-
        // -- -- -* --
        // -- -- *- --
        //
        // -- -* -- --
        // -- *- -- --
        // -* -- -- --
        // *- -- -- --
        let mut grid = GridCoords::new(4 * 2, 2 * 4);
        let coords = [
            (0., -3.),
            (1., -2.),
            (2., -1.),
            (3., 0.),
            (4., 1.),
            (5., 2.),
            (6., 3.),
            (7., 4.),
        ]
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect::<CartesianCoords>();
        grid.set_points(&coords);

        #[rustfmt::skip]
        let expected = vec![
            false, false, false, false, false, false, false,  true,
            false, false, false, false, false, false,  true, false,
            false, false, false, false, false,  true, false, false,
            false, false, false, false,  true, false, false, false,
            false, false, false,  true, false, false, false, false,
            false, false,  true, false, false, false, false, false,
            false,  true, false, false, false, false, false, false,
             true, false, false, false, false, false, false, false,
        ];

        assert_eq!(0., coords.bounds.x.min);
        assert_eq!(7., coords.bounds.x.max);
        assert_eq!(-3., coords.bounds.y.min);
        assert_eq!(4., coords.bounds.y.max);

        let dots = grid.into_dots();
        assert_eq!(expected, dots);

        // let expected_img = Framebuffer::new(&expected, 8, 8).to_string();
        let actual_img = Framebuffer::new(&dots, 8, 8).to_string();
        insta::assert_snapshot!(actual_img);
    }

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

    #[test]
    fn check_multiple_waves() {
        let mut grid = GridCoords::new(26 * 2 - 1, 10 * 4);
        let series_1 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::cos(x / 5.)));
        let series_2 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::sin(x / 4.)));
        let series_3 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::sin(x / 2.)));
        let mut series = series_1
            .chain(series_2)
            .chain(series_3)
            .collect::<Vec<_>>();
        let coords = CartesianCoords::new_with_bounds(series, CartesianBounds::new())
        series.bounds.x.min = -8. * std::f64::consts::PI;
        series.bounds.x.max = 8. * std::f64::consts::PI;
        series.bounds.y.min = -1.;
        series.bounds.y.max = 1.;

        grid.set_points(&series);

        let width = grid.width;
        let height = grid.height;
        let grid_values = grid.into_dots();
        let graph = Framebuffer::new(&grid_values, width, height);

        insta::assert_display_snapshot!(graph);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn check_big_thing() {
        let expected = vec![
            false, false, false, true, false, false, true, true, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, true, true,
            true, false, true, false, false, true, true, false, false, false, false, false, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, true, false, true, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, false, true, false, false, false, false, true, false,
            false, false, false, true, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false, true, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, false, true, true, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, true, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false, false, false, false, true, false, false,
            false, false, false, true, false, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, true, false, false, false, false, false, true, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true, false, true, false, false, false, false, false, false,
            false, true, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, true, false, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false, false, false, false, false, false, true, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, true,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, true,
            false, false, false, true, false, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false, false, false, false, false,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, true, false, false, false, true, false, true, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true, true, false, false, true, false, true, false, false, false,
            false, false, false, false, false, false, false, false, true, false, true, true, false,
            false, false, false, false, false, false, false, true, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false, false,
            false, false, true, false, false, true, true, false, true, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            true, false, false, false, false, false, false, false, true, true, false, false, true,
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, true, true, false, false, true, false, false, true,
            false, false, false, false,
        ];

        insta::assert_snapshot!(Framebuffer::new(&expected, 26 * 2, 10 * 4).to_string());
    }
}
