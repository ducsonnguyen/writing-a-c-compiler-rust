use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(about = "A C compiler")]
struct Args {
    /// C source file to compile
    file: PathBuf,

    /// Run the lexer, but stop before parsing
    #[arg(long, group = "stage")]
    lex: bool,

    /// Run the lexer and parser, but stop before assembly generation
    #[arg(long, group = "stage")]
    parse: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(long, group = "stage")]
    codegen: bool,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.file.extension().and_then(|e| e.to_str()) != Some("c") {
        return Err("input file must have a .c extension".into());
    }

    // TODO: preprocess the source file

    let _source = std::fs::read_to_string(&args.file)
        .map_err(|e| format!("could not read {}: {e}", args.file.display()))?;

    // TODO: lex
    if args.lex {
        return Ok(());
    }

    // TODO: parse
    if args.parse {
        return Ok(());
    }

    // TODO: codegen
    if args.codegen {
        return Ok(());
    }

    // TODO: emit assembly to .s file

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
