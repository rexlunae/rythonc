use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

use clap::Parser;
use python_ast::{parse, symbols::SymbolTableScopes, CodeGen, CodeGenContext, PythonOptions};
use rust_format::{Formatter, RustFmt};

use anyhow::Result;

// Set up the fern logging facility.
fn setup_logger(
    level: log::LevelFilter,
    log_file: Option<String>,
) -> std::result::Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}:{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                record.line().unwrap(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout());

    match log_file {
        None => {}
        Some(s) => logger = logger.chain(fern::log_file(s)?),
    };
    logger.apply()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The output file.")]
    output: Option<PathBuf>,
    #[arg()]
    inputs: Vec<PathBuf>,

    #[clap(long, short, action, help = "Nicely format the output.")]
    pretty: bool,

    #[clap(
        long,
        short,
        action,
        help = "Don't actually compile, just output the ast represented as a Rust data structure."
    )]
    ast_only: bool,

    #[clap(
        long,
        short,
        action,
        help = "Don't actually compile, just output the symbols."
    )]
    symbols_only: bool,

    #[clap(long, short, action, help="Sets the log level. Values are: off,error,warn,info,debug,trace", default_value_t=log::LevelFilter::Warn)]
    log_level: log::LevelFilter,

    #[clap(long, action, help = "Write log events to this file.")]
    log_file: Option<String>,

    #[clap(long, short, action, help = "Compile without stdpython.")]
    nostd: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    setup_logger(args.log_level, args.log_file)?;
    let options = PythonOptions {
        with_std_python: !args.nostd,
        ..Default::default()
    };

    let mut output_list = Vec::new();

    for input in args.inputs {
        let module_name = format!("{}", input.to_string_lossy());
        let py = read_to_string(input.clone())?;
        let ast = parse(&py, &module_name)?;

        let output = if args.ast_only {
            if args.pretty {
                format!("{:#?}", ast)
            } else {
                format!("{:?}", ast)
            }
        } else {
            let symbols = ast.clone().find_symbols(SymbolTableScopes::new());
            if args.symbols_only {
                if args.pretty {
                    format!("{:#?}", symbols)
                } else {
                    format!("{:?}", symbols)
                }
            } else {
                let rust = ast
                    .to_rust(
                        CodeGenContext::Module(module_name),
                        options.clone(),
                        symbols.clone(),
                    )
                    .expect("converting to Rust");
                if args.pretty {
                    let unformatted = rust.to_string();
                    RustFmt::default().format_str(unformatted)?
                } else {
                    format!("{}", rust)
                }
            }
        };
        output_list.push(output);
    }

    let file_output = output_list.join("");

    match args.output {
        Some(f) => {
            let mut file = File::create(f)?;
            file.write_all(file_output.as_bytes())?;
        }
        None => println!("{}", file_output),
    }

    Ok(())
}
