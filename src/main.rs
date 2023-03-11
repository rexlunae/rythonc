use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    #[arg(last = true)]
    inputs: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
