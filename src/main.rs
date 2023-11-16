use braille::Opt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::new();
    braille::run(opt)
}
