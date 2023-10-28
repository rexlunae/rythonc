# rythonc

A compiler for Rython, a Python-like language. It can either generate Rust code, or an Abstract Syntax Tree (AST).

## Installation

If you already have `rust` and `cargo` installed, you can just install `rythonc` with cargo.

```bash
cargo install rythonc
```

## Useage

The useage can be displayed at the command line by running `rythonc -h`.

By default, the output is fairly minimal. If you would like more human-readable output, use the `-p` flag.

By default, the output is printed to stdout. To save it in a file, use the `-o` flag.

```
Usage: rythonc [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...

Options:
  -o, --output <OUTPUT>        The output file.
  -p, --pretty                 Nicely format the output.
  -a, --ast-only               Don't actually compile, just output the ast.
  -l, --log-level <LOG_LEVEL>  Sets the log level. Values are: off,error,warn,info,debug,trace [default: WARN]
      --log-file <LOG_FILE>    Write log events to this file.
  -h, --help                   Print help
  -V, --version                Print version
  ```
