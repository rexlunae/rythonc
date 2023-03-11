use std::path::{Path, PathBuf};
use std::fs::read_to_string;

use clap::Parser;
use python_ast::{parse, PythonContext, CodeGen};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help="The output file.")]
    output: Option<PathBuf>,
    #[arg()]
    inputs: Vec<PathBuf>,

    #[clap(long, short, action, help="Don't actually compile, just output the ast.")]
    ast_only: bool,
}

fn main() {
    let args = Args::parse();
    let mut ctx = PythonContext::default();

    for input in args.inputs {
        let py = read_to_string(input).unwrap();
        let ast = parse(&py, "__main__").unwrap();

        let output = if args.ast_only {
            format!("{:?}", ast)
        } else {
            format!("{}", ast.to_rust(&mut ctx).unwrap())
        };
        println!("{}", output);
    }

}
