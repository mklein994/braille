use std::io::{LineWriter, Write};

use super::Char;
use crate::Config;
use crate::InputLine;
use crate::InputLineSinglable;
use crate::graph::braille::Brailleish;
use crate::graph::{BarGraphable, DotArrayable, Graphable, RowBuildable};
use crate::opt::ValueIter;

pub struct Lines {
    config: Config,
}

impl From<Config> for Lines {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl RowBuildable for Lines {}

impl BarGraphable<Option<f64>> for Lines {}
impl Graphable<Option<f64>> for Lines
where
    Self: BarGraphable<Option<f64>>,
{
    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        input_lines: ValueIter<Option<f64>>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let mut input_lines = input_lines.into_iter();
        let minimum = <Self as Graphable<Option<f64>, Config>>::minimum(self);
        let maximum = <Self as Graphable<Option<f64>, Config>>::maximum(self);
        let style = <Self as Graphable<Option<f64>, Config>>::style(self);
        let width = <Self as BarGraphable<Option<f64>>>::width(self);

        let min = 1; // reserve an empty line for null values
        let max = width * 2; // braille characters are 2 dots wide
        let scale = |value: f64| Self::scale(value, minimum, maximum, min, max);

        // Clamp where 0 would fit to be inside the output range
        let zero = if minimum > 0. {
            min
        } else if maximum < 0. {
            max
        } else {
            scale(0.)
        };

        // Each braille character is 4 dots tall
        let mut buffer = [vec![], vec![], vec![], vec![]];
        let mut has_more_lines = true;
        while has_more_lines {
            for buffer_line in &mut buffer {
                let input_line = input_lines.next();
                if input_line.is_none() {
                    has_more_lines = false;
                }

                if let Some(new_line) = input_line
                    .transpose()?
                    .and_then(InputLine::into_inner)
                    .map(scale)
                    .map(|value| Self::into_dot_groups(value, zero, style))
                {
                    *buffer_line = new_line;
                }
            }

            if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
                let transposed = Self::assemble_row(&buffer);
                let braille_line = transposed
                    .into_iter()
                    .map(|x| Char::new(x).as_str())
                    .collect::<String>();
                writeln!(writer, "{braille_line}")?;
            }

            buffer.fill(vec![]);
        }

        Ok(())
    }
}

impl Brailleish<2> for Lines {}
impl DotArrayable for Lines {}

impl<const N: usize> BarGraphable<[Option<f64>; N]> for Lines {}
impl<const N: usize> Graphable<[Option<f64>; N]> for Lines {
    fn config(&self) -> &Config {
        &self.config
    }

    fn print_graph<W: Write>(
        &self,
        input_lines: ValueIter<[Option<f64>; N]>,
        mut writer: LineWriter<W>,
    ) -> anyhow::Result<()> {
        let mut input_lines = input_lines.into_iter();
        let minimum = <Self as Graphable<[Option<f64>; N], Config>>::minimum(self);
        let maximum = <Self as Graphable<[Option<f64>; N], Config>>::maximum(self);
        let width = <Self as BarGraphable<[Option<f64>; N]>>::width(self);
        let style = <Self as Graphable<[Option<f64>; N], Config>>::style(self);

        let min = 1;
        let max = width * 2;
        let scale = |value| Self::scale(value, minimum, maximum, min, max);

        let mut buffer = [vec![], vec![], vec![], vec![]];
        let mut has_more_lines = true;
        while has_more_lines {
            for buffer_line in &mut buffer {
                let input_line = input_lines.next();
                if input_line.is_none() {
                    has_more_lines = false;
                }

                if let Some(new_line) = input_line
                    .transpose()?
                    .and_then(|x| {
                        if x.as_single_iter().all(Option::is_none) {
                            None
                        } else {
                            let line = x.into_iter().map(|x| scale(x.unwrap())).collect::<Vec<_>>();
                            Some(<[_; N]>::try_from(line).unwrap())
                        }
                    })
                    .map(|x| Self::into_dot_array_groups(x, style))
                {
                    *buffer_line = new_line;
                }
            }

            if has_more_lines || buffer.iter().any(|x| !x.is_empty()) {
                let transposed = Self::assemble_row(&buffer);
                let braille_line = transposed
                    .into_iter()
                    .map(|x| Char::new(x).as_str())
                    .collect::<String>();
                writeln!(writer, "{braille_line}")?;
            }

            buffer.fill(vec![]);
        }

        Ok(())
    }
}
