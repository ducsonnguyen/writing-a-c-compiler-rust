use crate::assembly_ast::{Function, Instruction, Operand, Program};
use std::fmt::Write;
use std::io;
use std::path::Path;

pub fn emit(program: &Program, output: &Path) -> io::Result<()> {
    let mut out = String::new();
    emit_program(&mut out, program);
    std::fs::write(output, out)
}

fn emit_program(out: &mut String, program: &Program) {
    let Program::Program(func) = program;
    emit_function(out, func);
    if cfg!(target_os = "linux") {
        writeln!(out, "\t.section .note.GNU-stack,\"\",@progbits").unwrap();
    }
}

fn mangle(name: &str) -> String {
    if cfg!(target_os = "macos") {
        format!("_{name}")
    } else {
        name.to_string()
    }
}

fn emit_function(out: &mut String, func: &Function) {
    let label = mangle(&func.name);
    writeln!(out, "\t.globl {label}").unwrap();
    writeln!(out, "{label}:").unwrap();
    for instr in &func.instructions {
        emit_instruction(out, instr);
    }
}

fn emit_instruction(out: &mut String, instr: &Instruction) {
    match instr {
        Instruction::Mov(src, dst) => {
            writeln!(out, "\tmovl {}, {}", operand_str(src), operand_str(dst)).unwrap();
        }
        Instruction::Ret => {
            writeln!(out, "\tret").unwrap();
        }
    }
}

fn operand_str(operand: &Operand) -> String {
    match operand {
        Operand::Imm(n) => format!("${n}"),
        Operand::Register => "%eax".to_string(),
    }
}
