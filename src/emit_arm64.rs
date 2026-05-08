use crate::assembly_ast::{Function, Instruction, Operand, Program, Reg, UnaryOperator};
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
    writeln!(out, "\tstp x29, x30, [sp, #-16]!").unwrap();
    writeln!(out, "\tmov x29, sp").unwrap();
    for instr in &func.instructions {
        emit_instruction(out, instr);
    }
}

fn emit_instruction(out: &mut String, instr: &Instruction) {
    match instr {
        // ARM64 operand order is dst, src (opposite of AT&T)
        Instruction::Mov(src, dst) => {
            writeln!(out, "\tmov {}, {}", operand_str(dst), operand_str(src)).unwrap();
        }
        Instruction::Unary(op, operand) => {
            // ARM64 unary instructions take separate dst, src; here both are the same
            let s = operand_str(operand);
            writeln!(out, "\t{} {s}, {s}", unop_mnemonic(op)).unwrap();
        }
        Instruction::AllocateStack(n) => {
            writeln!(out, "\tsub sp, sp, #{n}").unwrap();
        }
        Instruction::Ret => {
            writeln!(out, "\tmov sp, x29").unwrap();
            writeln!(out, "\tldp x29, x30, [sp], #16").unwrap();
            writeln!(out, "\tret").unwrap();
        }
    }
}

fn unop_mnemonic(op: &UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Neg => "neg",
        UnaryOperator::Not => "mvn",
    }
}

fn operand_str(operand: &Operand) -> String {
    match operand {
        Operand::Imm(n) => format!("#{n}"),
        Operand::Reg(r) => reg_str(r).to_string(),
        Operand::Stack(n) => format!("[x29, #{n}]"),
        Operand::Pseudo(name) => unreachable!("pseudo {name} should have been replaced before emission"),
    }
}

fn reg_str(reg: &Reg) -> &'static str {
    match reg {
        Reg::RV => "w0",
        Reg::Scratch1 => "w10",
    }
}
