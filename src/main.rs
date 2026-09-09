use bf::*;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "bf", about = "A brainfuck interpreter")]
struct Args {
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let src = fs::read_to_string(&args.file)?;
    let bfs = parse(src.as_bytes(), &mut 0)?;
    let bfs = bf_to_pbf(Preproc::new().preproc(bfs)?);

    let mut ctx = Context::new();
    ctx.run(bfs)?;

    Ok(())
}
