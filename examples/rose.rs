use std::{
    fs::File,
    io::{BufReader, LineWriter},
};

use braille::Opt;

fn main() -> anyhow::Result<()> {
    // braille -G-1:1 -g
    let file = File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/rose.txt"))?;
    let mut stdin = BufReader::new(file);
    // let mut stdin = std::io::stdin().lock();
    let opt = Opt::try_new_from_reader(&mut stdin, ["braille", "-m", "--use-full-default-height"])?;
    let writer = LineWriter::new(std::io::stdout());

    braille::grid::print_graph(opt, stdin, writer)?;

    Ok(())
}
