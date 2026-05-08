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
    writeln!(out, "\tpushq %rbp").unwrap();
    writeln!(out, "\tmovq %rsp, %rbp").unwrap();
    for instr in &func.instructions {
        emit_instruction(out, instr);
    }
}

fn emit_instruction(out: &mut String, instr: &Instruction) {
    match instr {
        Instruction::Mov(src, dst) => {
            writeln!(out, "\tmovl {}, {}", operand_str(src), operand_str(dst)).unwrap();
        }
        Instruction::Unary(op, operand) => {
            writeln!(out, "\t{}l {}", unop_mnemonic(op), operand_str(operand)).unwrap();
        }
        Instruction::AllocateStack(n) => {
            writeln!(out, "\tsubq ${n}, %rsp").unwrap();
        }
        Instruction::Ret => {
            writeln!(out, "\tmovq %rbp, %rsp").unwrap();
            writeln!(out, "\tpopq %rbp").unwrap();
            writeln!(out, "\tret").unwrap();
        }
    }
}

fn unop_mnemonic(op: &UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Neg => "neg",
        UnaryOperator::Not => "not",
    }
}

fn operand_str(operand: &Operand) -> String {
    match operand {
        Operand::Imm(n) => format!("${n}"),
        Operand::Reg(r) => reg_str(r).to_string(),
        Operand::Stack(n) => format!("{n}(%rbp)"),
        Operand::Pseudo(name) => unreachable!("pseudo {name} should have been replaced before emission"),
    }
}

fn reg_str(reg: &Reg) -> &'static str {
    match reg {
        Reg::RV => "%eax",
        Reg::Scratch1 => "%r10d",
    }
}
