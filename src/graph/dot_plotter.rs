pub struct Plot<const N: usize> {
    inner: Vec<[bool; N]>,
}

impl<const N: usize> IntoIterator for Plot<N> {
    type Item = [bool; N];

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<const N: usize> Plot<N> {
    fn new(values: &[usize]) -> Self {
        let mut values = values.to_vec();
        values.sort_unstable();
        let max = *values.iter().max().unwrap();
        let base = max / N * N;
        let len = if base < max { base + N } else { base };

        let mut bits: Vec<bool> = vec![false; len];
        for value in values {
            bits[value] = true;
        }

        let inner: Vec<[bool; N]> = bits
            .chunks_exact(N)
            .map(|x| x.try_into().unwrap())
            .collect();

        Self { inner }
    }
}

trait Plotable {
    type PlotItem;

    fn plot_prefix(self, length: usize) -> PlotPrefix<Self>
    where
        Self: Sized,
    {
        PlotPrefix::new(self, length)
    }
}

struct PlotPrefix<I> {
    iter: I,
    length: usize,
}

impl<I> PlotPrefix<I> {
    fn new(iter: I, length: usize) -> Self {
        Self { iter, length }
    }
}

impl<I: Iterator> Iterator for PlotPrefix<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct Plotter {
    values: Vec<u16>,
}

impl Plotter {
    fn new(values: &[u16]) -> Self {
        let mut values = values.to_vec();
        values.sort_unstable();
        values.reverse();
        Self { values }
    }
}

impl Iterator for Plotter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        // y = sin(x / 2)
        //
        //  abcdefghijklmnopqrstuvwxyz
        // 0⠀⡈⠄⠀⠀⠀⠀⠐⠁⠀⠀⠀⠀⠀⠑⠀⠀⠀⠀⠀⠌⡀⠀⠀⠀⠀
        // 4⠀⠀⠀⠀⠀⠀⠀⡀⠈⠀⠀⠀⠀⠈⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        // 8⠠⠀⠈⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠈⠀⠠⠀⠀⠀⠀
        //12⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        //16⡀⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀⠀⠀⠀⠀
        //20⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠄
        //24⠀⠀⠀⠠⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠠⠀⠀⠀⠀⠀⠀⠀
        //28⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⢀⠀
        //32⠀⠀⠀⠀⠄⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠄⠀⠀⠀⠀⠀⠀⠀
        //36⠀⠀⠀⠀⢀⠄⠀⠀⠀⠀⠐⡐⠀⠀⠀⠀⠀⢄⠀⠀⠀⠀⠀⢁⠂⠀

        // y = sin(x / 4)
        // y = cos(x / 5)
        //  abcdefghijklmnopqrstuvwxyz
        // 0⠀⠀⡐⠉⠄⠀⠀⠀⠀⠀⠀⠠⠉⠑⣀⠊⠡⠀⠀⠀⠀⠀⠀⠀⠀⠀
        // 4⠀⠠⠀⠀⠈⠀⠀⠀⠀⠀⠠⠁⠀⠀⡠⠀⠀⢁⠀⠀⠀⠀⠀⠀⠀⠀
        // 8⠀⠄⠀⠀⠀⠁⠀⠀⠀⠀⠂⠀⠀⢀⠀⠂⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀
        //12⢂⠀⠀⠀⠀⠈⠀⠀⠀⠐⠀⠀⠀⡀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄
        //16⡐⠀⠀⠀⠀⠀⠂⠀⠀⠂⠀⠀⠀⠀⠀⠀⠁⠀⠈⠀⠀⠀⠀⠀⠠⠀
        //20⠀⠂⠀⠀⠀⠀⠠⠀⠐⠀⠀⠀⠈⠀⠀⠀⠈⡀⠀⠂⠀⠀⠀⠀⠄⠂
        //24⠀⠐⠀⠀⠀⠀⠀⠄⠁⠀⠀⠀⠂⠀⠀⠀⠀⢀⠀⠠⠀⠀⠀⠠⠠⠀
        //28⠀⠀⠂⠀⠀⠀⠀⣈⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⡀⠀⠄⠀⠀⠄⠄⠀
        //32⠀⠀⠈⡀⠀⠀⠠⠀⠄⠀⠀⠂⠀⠀⠀⠀⠀⠀⠠⠀⠠⠀⡐⠠⠀⠀
        //36⠀⠀⠀⠐⢄⡠⠁⠀⠐⣀⠌⠀⠀⠀⠀⠀⠀⠀⠀⠡⣀⠶⡠⠂⠀⠀

        use crate::graph::{braille::Char, RowBuildable};

        struct Foo;
        impl RowBuildable for Foo {}

        impl Char {
            fn from_dot_string<W: std::fmt::Write>(mut writer: W, s: &str) -> std::fmt::Result {
                let lines: Vec<_> = s.lines().skip(1).filter(|x| !x.is_empty()).collect();
                assert!(lines.len() % 4 == 0, "actual len: {}", lines.len());

                for block_line in lines.chunks_exact(4) {
                    let mut line = [vec![], vec![], vec![], vec![]];
                    for (i, row) in block_line.iter().enumerate() {
                        let pairs: Vec<_> = row
                            .replace(|c: char| ![' ', '-', '*'].contains(&c), "")
                            .split_ascii_whitespace()
                            .map(|pair| match pair {
                                "--" => [false, false],
                                "-*" => [false, true],
                                "*-" => [true, false],
                                "**" => [true, true],
                                x => panic!("Invalid braille pair: {x:?}"),
                            })
                            .collect();
                        line[i] = pairs;
                    }

                    for block in Foo::assemble_row(&line) {
                        write!(writer, "{}", Self::new(block))?;
                    }

                    writeln!(writer)?;
                }

                Ok(())
            }
        }

        let mut buf = String::new();
        Char::from_dot_string(
            &mut buf,
            "\
#    aa bb cc dd ee ff gg hh ii jj kk ll mm nn oo pp qq rr ss tt uu vv ww xx yy zz
# 0  -- -* -- ** -- -- -- -- *- -- -- -- ** *- *- -* *- -- -- -- -* -- -- -- -- --
# 1  -- -- -* -- -- -- -- -* -- -- -- -- -- -* -* *- -- -- -- -- -- -- -- -- -- --
# 2  -- -- *- -- *- -- -- -- -- -- -- -* -- -- -- -- -* -- -- -- *- -- -- -- -- --
# 3  -- *- *- -- -- -- -- -- -- -- -- -- -- -- ** -- -- -- -- -- -- *- -- -- -- --

# 4  -- -- -- -- -* -- -- -- -* -- -- *- -- -* -- -- -- *- -- -- -- -- -- -- -- --
# 5  -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 6  -- -* -- -- -- -- -- -- -- -- -* -- -- -- -* -- -- -- -- -- -- -- -- -- -- --
# 7  -- -- -- -- -- -- -- *- -- -- -- -- -- -- *- *- -- -* -- -- -- -- -- -- -- --

# 8  -- -- -* -- -- *- -- -- -- -- -- -- -- -- -- -- -- -- -- -* -- -- -- -- -- --
# 9  -- -- -- -- -- -- -- -- -- -- *- -- -- -- -- *- -- -- -- -- -- -- -- -- -- --
# 10 -* *- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -* -- -- -- --
# 11 -- -- -- -- -- -- -- -- -- *- -- -- -- ** -- -- -- -- *- -- -- -- -- -- -- --

# 12 -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- --
# 13 *- -- -- -- -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 14 -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- *-
# 15 -* -- -- -- -- -- -* -- -- -- -- -- -- *- -- -* -- -- -- -- -- -- -- -- -- --

# 16 -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- *- -- -* -- -- -- -- -- -- --
# 17 -* -- -- *- -- -- *- -- -- *- -- -- -- -- -- -- -- -- -- *- -- -- -- -- -- --
# 18 -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -* --
# 19 *- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --

# 20 -- -- -- -- -- -- -- -- -- -- -- -- -* -- -- -- -* -- -- -- -- -- *- -- -- --
# 21 -- *- -- -- -- -- -- -- -* -* -- -- -* -- -- -- -- -- -- *- -- -- -- -- -- *-
# 22 -- -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- *- *-
# 23 -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- *- -- -- -- -- -- -- -- --

# 24 -- -- -- -- -- -- -- -- *- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 25 -- -* -- -- -- -- *- -- -- -- -- -- *- -- -- -- *- -- -- -- -- -- -- -- -- --
# 26 -- -- -- -* -- -- -- *- -- -- -- -- -- -- -- -- -- -- -* -* -- -- -- -* -* --
# 27 -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -* -- -- -- -- -- -- -- --

# 28 -- -- -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 29 -- -- *- -- -- -- -- -- -- -- -- -* -- -- -- -- -- -- -- -- -- -- -* -- -- --
# 30 -- -- -- -- -- -- -- -- -- -- *- -- *- -- -- -- -- -- -- -- *- -- -- *- *- --
# 31 -- -- -- -- -- -- -- ** -- -- -- -- -- -- -- -- -- -- *- -- -- -- -- -- -* --

# 32 -- -- -* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
# 33 -- -- -- -- -- -* -- -- -- -- -- *- -- -- -- -- -* -- -- -- -- -- -* -- -- --
# 34 -- -- -- -- *- -- -* -- *- -- -- -- -- -- -- -- -- -- ** -- -* -- -- -* -- --
# 35 -- -- -- *- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- *- -- -- --

# 36 -- -- -- -- -- -- *- -- -- -- -* -- -- -- -- -- -- -- -- *- -- -- -- *- -- --
# 37 -- -- -- -* -- -- -- -- -* -- -* -* -- -- -- -- -- -- -- -- -- ** -- *- *- --
# 38 -- -- -- -- *- ** -- -- -- -- *- -- -- -- -- -- -- *- -- -* -- ** -* -- -- --
# 39 -- -- -- -- -* *- -- -- -- ** -- *- -- -- -- -- -- -* -- -- ** -- *- -* -- --
",
        )
        .unwrap();

        insta::assert_snapshot!(buf);
    }
}
