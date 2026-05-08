mod assembly_ast;
mod ast;
mod codegen;
mod emit_arm64;
mod emit_x86_64;
mod lexer;
mod parser;
mod tacky;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(ValueEnum, Clone, Default)]
enum Arch {
    #[default]
    #[value(name = "x86_64")]
    X86_64,
    #[value(name = "arm64")]
    Arm64,
}

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

    /// Run lexing, parsing, and TACKY IR generation, but stop before assembly generation
    #[arg(long, group = "stage")]
    tacky: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(long, group = "stage")]
    codegen: bool,

    /// Compile to assembly but do not link
    #[arg(short = 'S', group = "stage")]
    assembly: bool,

    /// Target architecture
    #[arg(long, default_value = "x86_64")]
    arch: Arch,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Preprocess the source file
    let preprocessed = args.file.with_extension("i");
    let status = std::process::Command::new("gcc")
        .args(["-E", "-P"])
        .arg(&args.file)
        .arg("-o")
        .arg(&preprocessed)
        .status()
        .map_err(|e| format!("failed to run gcc preprocessor: {e}"))?;
    if !status.success() {
        return Err("preprocessing failed".into());
    }

    let source = std::fs::read_to_string(&preprocessed)
        .map_err(|e| format!("could not read {}: {e}", preprocessed.display()))?;
    std::fs::remove_file(&preprocessed).ok();

    let tokens = lexer::lex(&source)?;
    if args.lex {
        return Ok(());
    }

    let program = parser::parse(&tokens)?;
    if args.parse {
        println!("{program}");
        return Ok(());
    }

    if args.tacky {
        let tacky_program = tacky::gen_program(program);
        println!("{tacky_program}");
        return Ok(());
    }

    let asm = codegen::gen_program(program);
    if args.codegen {
        println!("{asm}");
        return Ok(());
    }

    let asm_file = args.file.with_extension("s");
    match args.arch {
        Arch::X86_64 => emit_x86_64::emit(&asm, &asm_file),
        Arch::Arm64 => emit_arm64::emit(&asm, &asm_file),
    }
    .map_err(|e| format!("failed to write {}: {e}", asm_file.display()))?;
    if args.assembly {
        return Ok(());
    }

    let exe_file = args.file.with_extension("");
    let mut link_cmd = std::process::Command::new("gcc");
    link_cmd.arg(&asm_file).arg("-o").arg(&exe_file);
    if cfg!(target_os = "macos") {
        let arch_str = match args.arch {
            Arch::X86_64 => "x86_64",
            Arch::Arm64 => "arm64",
        };
        link_cmd.args(["-arch", arch_str]);
    }
    let status = link_cmd
        .status()
        .map_err(|e| format!("failed to run gcc assembler/linker: {e}"))?;
    std::fs::remove_file(&asm_file).ok();
    if !status.success() {
        return Err("assembly and linking failed".into());
    }

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
