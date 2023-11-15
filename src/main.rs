use braille::Opt;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    match &opt.file {
        Some(path) => {
            let lines = braille::get_lines_from_file(path)?;
            braille::print_lines(&opt, lines)?;
        }
        None => {
            let lines = braille::get_lines();
            braille::print_lines(&opt, lines)?;
        }
    }

    Ok(())
}
