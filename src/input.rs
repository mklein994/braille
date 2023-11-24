use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct InputLine<T>(T);

impl<T> InputLine<T> {
    fn parse_value(s: &str) -> Result<Option<f64>, <f64 as FromStr>::Err> {
        if s.is_empty() || s == "null" {
            Ok(None)
        } else {
            Some(s.parse()).transpose()
        }
    }
}

pub trait InputLineSinglable {
    type Iter: Iterator<Item = Option<f64>>;

    fn as_single_iter(&self) -> Self::Iter;
}

impl InputLineSinglable for InputLine<Option<f64>> {
    type Iter = std::vec::IntoIter<Option<f64>>;

    fn as_single_iter(&self) -> Self::Iter {
        vec![self.0].into_iter()
    }
}

impl<const N: usize> InputLineSinglable for InputLine<[Option<f64>; N]> {
    type Iter = std::array::IntoIter<Option<f64>, N>;

    fn as_single_iter(&self) -> Self::Iter {
        self.0.into_iter()
    }
}

impl InputLine<Option<f64>> {
    pub fn into_inner(self) -> Option<f64> {
        self.0
    }
}

impl FromStr for InputLine<Option<f64>> {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Self::parse_value(s)?))
    }
}

impl<const N: usize> FromStr for InputLine<[Option<f64>; N]> {
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

impl<T> IntoIterator for InputLine<T>
where
    T: IntoIterator<Item = Option<f64>>,
{
    type Item = Option<f64>;

    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub type InputLineResult<T> = Result<InputLine<T>, <InputLine<T> as FromStr>::Err>;

pub struct InputLines<T>
where
    InputLine<T>: FromStr,
{
    iter: Box<dyn Iterator<Item = InputLineResult<T>>>,
}

impl<T> Iterator for InputLines<T>
where
    InputLine<T>: FromStr,
{
    type Item = InputLineResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T: 'static> InputLines<T>
where
    InputLine<T>: FromStr,
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
        let first = first_line.map(|line| vec![Ok(line)]).unwrap_or_default();

        Self {
            iter: Box::new(
                first
                    .into_iter()
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
        let expected: Vec<InputLineResult<[Option<f64>; 3]>> = vec![
            Ok(InputLine([Some(1.), Some(2.), Some(3.)])),
            Ok(InputLine([Some(4.), Some(5.), Some(6.)])),
        ];

        let iter = InputLines::<[Option<f64>; 3]>::from_buf_reader(None, Cursor::new(input));
        let actual: Vec<_> = iter.collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_lines_iter_one() {
        use std::io::Cursor;

        let input = "1\n2\n3\n4\n5\n6";
        let expected: Vec<InputLineResult<Option<f64>>> = vec![
            Ok(InputLine(Some(1.))),
            Ok(InputLine(Some(2.))),
            Ok(InputLine(Some(3.))),
            Ok(InputLine(Some(4.))),
            Ok(InputLine(Some(5.))),
            Ok(InputLine(Some(6.))),
        ];

        let iter = InputLines::<Option<f64>>::from_buf_reader(None, Cursor::new(input));
        let actual: Vec<_> = iter.collect();
        assert_eq!(expected, actual);
    }
}
