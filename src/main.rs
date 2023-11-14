use braille::Opt;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let lines = braille::get_lines();

    braille::print_lines(&opt, lines)?;

    Ok(())
}
