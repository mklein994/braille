mod bounds;

use crate::opt::CharType;
use crate::util;
use bounds::{CartesianBound, CartesianBounds};
use braillefb::{Framebuffer, FramebufferStyle};
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::BTreeSet as HashSet;
use std::io::{LineWriter, Write};

type DotUnit = u16;

struct CartesianPoints {
    bounds: CartesianBounds,
    inner: Vec<Point>,
}

impl CartesianPoints {
    fn new_with_bounds(list: Vec<Point>, bounds: CartesianBounds) -> Self {
        Self {
            inner: list,
            bounds,
        }
    }

    fn new(list: Vec<Point>) -> Self {
        let bounds = list.iter().collect::<CartesianBounds>();

        Self {
            inner: list,
            bounds,
        }
    }
}

impl FromIterator<Point> for CartesianPoints {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut inner = vec![];
        let CartesianBound {
            min: mut x_min,
            max: mut x_max,
        } = CartesianBound::default();
        let CartesianBound {
            min: mut y_min,
            max: mut y_max,
        } = CartesianBound::default();
        for point in iter {
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
            inner.push(point);
        }

        Self {
            inner,
            bounds: CartesianBounds::new(x_min, x_max, y_min, y_max),
        }
    }
}

impl<'a> IntoIterator for &'a CartesianPoints {
    type Item = &'a Point;

    type IntoIter = std::slice::Iter<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for CartesianPoints {
    type Item = Point;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

struct GridDots {
    width: DotUnit,
    height: DotUnit,
    inner: HashSet<Dot>,
}

impl GridDots {
    pub fn new(width: DotUnit, height: DotUnit, capacity: usize) -> Self {
        // let inner = (0..height)
        //     .flat_map(move |y| (0..width).map(move |x| (Dot::new(x, y), false)))
        //     .collect::<HashMap<Dot, bool>>();
        // let inner = HashSet::with_capacity(usize::from(width) * usize::from(height));
        // let inner = HashSet::new();
        let inner = HashSet::with_capacity(capacity);

        // debug_assert_eq!(usize::try_from(width * height).unwrap(), inner.len());

        Self {
            width,
            height,
            inner,
        }
    }

    pub fn merge_points(&mut self, points: &CartesianPoints) {
        for point in points {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
            let x = util::scale(
                point.x,
                points.bounds.x.min,
                points.bounds.x.max,
                0.,
                f64::from(self.width - 1),
            )
            .round() as DotUnit;

            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
            let y = util::scale(
                point.y,
                points.bounds.y.min,
                points.bounds.y.max,
                0.,
                f64::from(self.height - 1),
            )
            .round() as DotUnit;

            let dot = Dot::new(x, y);
            // self.inner.entry(dand_modify).or_insert(|e| *e = true);
            self.inner.insert(dot);
        }
    }

    pub fn into_dots(self) -> Vec<bool> {
        let mut dots = Vec::with_capacity(usize::from(self.width * self.height));
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let dot = Dot::new(x, y);
                dots.push(self.inner.contains(&dot));
            }
        }

        dots
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Dot {
    x: DotUnit,
    y: DotUnit,
}

impl Dot {
    fn new(x: DotUnit, y: DotUnit) -> Self {
        Self { x, y }
    }
}

pub fn print_graph<W: Write>(
    opt: crate::Opt,
    reader: impl std::io::BufRead,
    mut writer: LineWriter<W>,
) -> anyhow::Result<()> {
    let grid_style = match opt.kind().char_type() {
        CharType::Octant => FramebufferStyle::Octants,
        CharType::Braille => FramebufferStyle::Braille,
        _ => FramebufferStyle::default(),
    };

    let grid = opt.grid.unwrap();
    let (width, height) = if grid.is_empty() {
        let (width, height) = crate::util::get_terminal_size().map(|(w, h)| {
            (
                DotUnit::from(w) * 2,
                DotUnit::from(h - DotUnit::from(!opt.use_full_default_height)) * 4,
            )
        })?;
        let square = width.min(height);
        (square, square)
    } else if grid.len() == 1 {
        (grid[0], grid[0])
    } else {
        (grid[0], grid[1])
    };

    let mut points: Vec<Point> = vec![];
    for line in reader.lines() {
        let line = line?;
        let (x, y) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
        points.push(Point::new(x.parse()?, y.parse()?));
    }

    let points = if opt.grid_bounds.or(opt.x_bounds).or(opt.y_bounds).is_some() {
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

        let bounds = builder.build_from_points(&points);
        CartesianPoints::new_with_bounds(points, bounds)
    } else {
        CartesianPoints::new(points)
    };
    let mut grid = GridDots::new(width, height, points.inner.len());
    grid.merge_points(&points);

    let dots = grid.into_dots();
    let fb = Framebuffer::new(&dots, width.into(), height.into()).with_style(grid_style);

    write!(writer, "{fb}")?;

    Ok(())
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
        let mut grid = GridDots::new(4 * 2, 2 * 4, 8);
        let points = [
            Point::new(0., -3.),
            Point::new(1., -2.),
            Point::new(2., -1.),
            Point::new(3., 0.),
            Point::new(4., 1.),
            Point::new(5., 2.),
            Point::new(6., 3.),
            Point::new(7., 4.),
        ]
        .into_iter()
        .collect::<CartesianPoints>();
        grid.merge_points(&points);

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

        assert!((points.bounds.x.min - 0.).abs() < f64::EPSILON);
        assert!((points.bounds.x.max - 7.).abs() < f64::EPSILON);
        assert!((points.bounds.y.min - -3.).abs() < f64::EPSILON);
        assert!((points.bounds.y.max - 4.).abs() < f64::EPSILON);

        let dots = grid.into_dots();
        assert_eq!(expected, dots);

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
        let series_1 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::cos(x / 5.)));
        let series_2 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::sin(x / 4.)));
        let series_3 = get_values()
            .into_iter()
            .map(|x| Point::new(x, f64::sin(x / 2.)));
        let points_list = series_1.chain(series_2).chain(series_3).collect();
        let points = CartesianPoints::new_with_bounds(
            points_list,
            CartesianBounds::new(
                -8. * std::f64::consts::PI,
                8. * std::f64::consts::PI,
                -1.,
                1.,
            ),
        );
        let mut grid = GridDots::new(26 * 2 - 1, 10 * 4, points.inner.len());
        grid.merge_points(&points);

        let width = grid.width;
        let height = grid.height;
        let grid_values = grid.into_dots();
        let graph = Framebuffer::new(&grid_values, usize::from(width), usize::from(height));

        insta::assert_snapshot!(graph);
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
