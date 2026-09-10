use anyhow::Result;
use bf::{Context, Preproc, bf_to_pbf, parse};
use clap::Parser;
use std::ffi;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "bf", about = "A brainfuck interpreter")]
pub(super) struct Args {
    pub(super) file: String,

    /// Provide input to the Brainfuck program from a file.
    #[clap(short, long)]
    pub(super) input: Option<String>,

    #[clap(long)]
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

    if let Some(input) = &args.input {
        redirect_stdin(input)?;
    }

    let bfs = parse(src.as_bytes(), &mut 0)?;
    let bfs = bf_to_pbf(Preproc::new().preproc(bfs)?);

    let mut ctx = Context::new();
    ctx.run(bfs)?;

    Ok(())
}

/// Some specific handling for macOS
#[cfg(target_os = "macos")]
fn redirect_stdin(path: &str) -> Result<()> {
    use libc::{O_RDONLY, STDIN_FILENO, close, dup2, open};

    let path = ffi::CString::new(path)?;

    unsafe {
        let fd = open(path.as_ptr(), O_RDONLY);
        if fd == -1 {
            anyhow::bail!("[BF]failed to open input file");
        }

        if dup2(fd, STDIN_FILENO) == -1 {
            close(fd);
            anyhow::bail!("[BF]failed to redirect stdin");
        }

        close(fd);
    }

    Ok(())
}

/// Use libc on non-macOS platforms
#[cfg(not(target_os = "macos"))]
use libc::freopen;

#[cfg(not(target_os = "macos"))]
unsafe extern "C" {
    static mut stdin: *mut libc::FILE;
}

#[cfg(not(target_os = "macos"))]
fn redirect_stdin(path: &str) -> Result<()> {
    let path = ffi::CString::new(path)?;

    unsafe {
        if freopen(path.as_ptr(), c"r".as_ptr(), stdin).is_null() {
            anyhow::bail!("[BF]freopen failed");
        }
    }

    Ok(())
}
