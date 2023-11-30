use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Line<T>(T);

impl<T> Line<T> {
    fn parse_value(s: &str) -> Result<Option<f64>, <f64 as FromStr>::Err> {
        if s.is_empty() || s == "null" {
            Ok(None)
        } else {
            Some(s.parse()).transpose()
        }
    }
}

pub trait LineSinglable<'a> {
    type Iter: Iterator<Item = &'a Option<f64>>;

    fn as_single_iter(&'a self) -> Self::Iter;
}

impl<'a> LineSinglable<'a> for Line<Option<f64>> {
    type Iter = std::iter::Once<&'a Option<f64>>;

    fn as_single_iter(&'a self) -> Self::Iter {
        std::iter::once(&self.0)
    }
}

impl<'a, const N: usize> LineSinglable<'a> for Line<[Option<f64>; N]> {
    type Iter = std::slice::Iter<'a, Option<f64>>;

    fn as_single_iter(&'a self) -> Self::Iter {
        self.0.iter()
    }
}

impl Line<Option<f64>> {
    pub fn into_inner(self) -> Option<f64> {
        self.0
    }
}

impl FromStr for Line<Option<f64>> {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Self::parse_value(s)?))
    }
}

impl<const N: usize> FromStr for Line<[Option<f64>; N]> {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<_> = s
            .splitn(N, |c: char| c.is_ascii_whitespace())
            .map(Self::parse_value)
            .collect::<Result<_, _>>()?;
        let len = line.len();
        Ok(Self(line.try_into().unwrap_or_else(|_| {
            panic!("Expected line with {N} values, found {len}")
        })))
    }
}

impl<T> IntoIterator for Line<T>
where
    T: IntoIterator<Item = Option<f64>>,
{
    type Item = Option<f64>;

    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub type LineResult<T> = Result<Line<T>, <Line<T> as FromStr>::Err>;

pub struct Lines<T>
where
    Line<T>: FromStr,
{
    iter: Box<dyn Iterator<Item = LineResult<T>>>,
}

impl<T> Iterator for Lines<T>
where
    Line<T>: FromStr,
{
    type Item = LineResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T: 'static> Lines<T>
where
    Line<T>: FromStr,
{
    pub fn try_from_path(first_line: Option<String>, path: Option<&Path>) -> anyhow::Result<Self> {
        match path {
            None => Ok(Self::from_buf_reader(first_line, std::io::stdin().lock())),
            Some(path) if path.as_os_str() == "-" => {
                Ok(Self::from_buf_reader(first_line, std::io::stdin().lock()))
            }
            Some(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                Ok(Self::from_buf_reader(first_line, reader))
            }
        }
    }

    pub fn from_buf_reader<R: BufRead + 'static>(first_line: Option<String>, reader: R) -> Self {
        Self {
            iter: Box::new(
                first_line
                    .into_iter()
                    .map(Ok)
                    .chain(reader.lines())
                    .map_while(Result::ok)
                    .map(|x| x.parse()),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lines_iter_many() {
        use std::io::Cursor;

        let input = "1 2 3\n4 5 6";
        let expected: Vec<LineResult<[Option<f64>; 3]>> = vec![
            Ok(Line([Some(1.), Some(2.), Some(3.)])),
            Ok(Line([Some(4.), Some(5.), Some(6.)])),
        ];

        let iter = Lines::<[Option<f64>; 3]>::from_buf_reader(None, Cursor::new(input));
        let actual: Vec<_> = iter.collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_lines_iter_one() {
        use std::io::Cursor;

        let input = "1\n2\n3\n4\n5\n6";
        let expected: Vec<LineResult<Option<f64>>> = vec![
            Ok(Line(Some(1.))),
            Ok(Line(Some(2.))),
            Ok(Line(Some(3.))),
            Ok(Line(Some(4.))),
            Ok(Line(Some(5.))),
            Ok(Line(Some(6.))),
        ];

        let iter = Lines::<Option<f64>>::from_buf_reader(None, Cursor::new(input));
        let actual: Vec<_> = iter.collect();
        assert_eq!(expected, actual);
    }
}
