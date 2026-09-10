use anyhow::Result;
use bf::{Context, Preproc, bf_to_pbf, parse};
use clap::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "bf", about = "A brainfuck interpreter")]
pub(super) struct Args {
    pub(super) file: String,

    #[clap(short, long)]
    pub(super) gcc: bool,
}

pub(super) fn build_app() -> Result<()> {
    let args = Args::parse();
    let mut src = fs::read_to_string(&args.file)?;

    if args.gcc {
        let output = Command::new("gcc").args(["-E", "-P", "-x", "c", &args.file]).output()?;
        if !output.status.success() {
            anyhow::bail!("gcc preprocess failed\n{}", String::from_utf8_lossy(&output.stderr));
        }
        src = String::from_utf8(output.stdout)?;
    }

    let bfs = parse(src.as_bytes(), &mut 0)?;
    let bfs = bf_to_pbf(Preproc::new().preproc(bfs)?);

    let mut ctx = Context::new();
    ctx.run(bfs)?;

    Ok(())
}
