use crate::assembly_ast::{self as asm};
use crate::ast::{Exp, Function, Program, Statement};

pub fn gen_program(program: Program) -> asm::Program {
    let Program::Program(func) = program;
    asm::Program::Program(gen_function(func))
}

fn gen_function(func: Function) -> asm::Function {
    asm::Function {
        name: func.name,
        instructions: gen_statement(func.body),
    }
}

fn gen_statement(stmt: Statement) -> Vec<asm::Instruction> {
    match stmt {
        Statement::Return(exp) => vec![
            asm::Instruction::Mov(gen_exp(exp), asm::Operand::Register),
            asm::Instruction::Ret,
        ],
    }
}

fn gen_exp(exp: Exp) -> asm::Operand {
    match exp {
        Exp::Constant(n) => asm::Operand::Imm(n),
        Exp::Unary(_, _) => todo!("codegen for unary operations"),
    }
}
