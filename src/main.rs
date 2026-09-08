use clap::Parser;

#[derive(Parser)]
#[command(name = "bf", about = "A brainfuck interpreter")]
struct Args {
    file: String,

    #[arg(short, long, default_value_t = 30000)]
    cells: usize,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let _args = Args::parse();
}
