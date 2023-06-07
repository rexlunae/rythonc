use std::path::{Path, PathBuf};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::time::SystemTime;

use clap::Parser;
use python_ast::{parse, PythonContext, CodeGen};
use rust_format::{Formatter, RustFmt};

// Set up the fern logging facility.
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help="The output file.")]
    output: Option<PathBuf>,
    #[arg()]
    inputs: Vec<PathBuf>,

    #[clap(long, short, action, help="Nicely format the output.")]
    pretty: bool,

    #[clap(long, short, action, help="Don't actually compile, just output the ast.")]
    ast_only: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;
    let args = Args::parse();
    let mut ctx = PythonContext::default();

    let mut output_list = Vec::new();

    for input in args.inputs {
        let py = read_to_string(input).unwrap();
        let ast = parse(&py, "__main__").unwrap();

        let output = if args.ast_only {
            if args.pretty {
                format!("{:#?}", ast)
            } else {
                format!("{:?}", ast)
            }
        } else {
            let rust = ast.to_rust(&mut ctx).unwrap();
            if args.pretty {
                let unformated = rust.to_string();
                RustFmt::default().format_str(unformated).unwrap()
            } else {
                format!("{}", rust)
            }
        };
        output_list.push(output);
    }

    let file_output = output_list.join("");

    match args.output {
        Some(f) => {
            let mut file = File::create(f).unwrap();
            file.write_all(file_output.as_bytes()).unwrap();
        }
        None => println!("{}", file_output),

    }

    Ok(())
}
