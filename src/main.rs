use braille::Opt;
use std::io::LineWriter;

fn main() -> anyhow::Result<()> {
    let opt = Opt::try_new()?;
    braille::run(opt, LineWriter::new(std::io::stdout()))
}
