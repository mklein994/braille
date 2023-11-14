use braille::Opt;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1).collect();
    let opt = Opt::from_args(args)?;
    let lines = braille::get_lines();

    braille::print_braille_lines(&opt, lines)?;

    Ok(())
}
