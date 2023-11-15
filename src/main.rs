use braille::Opt;

fn main() -> anyhow::Result<()> {
    let mut opt = Opt::new();
    braille::run(&mut opt)
}
