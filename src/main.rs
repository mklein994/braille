use braille::Opt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::try_new()?;
    braille::run(opt)
}
