use crate::assembly_ast::{self as asm};
use crate::tacky;
use std::collections::HashMap;

pub fn gen_program(program: tacky::Program) -> asm::Program {
    let asm_program = convert_program(program);
    let (asm_program, stack_size) = replace_pseudos(asm_program);
    fix_instructions(asm_program, stack_size)
}

// Pass 1: TACKY -> Assembly AST (with Pseudo operands)

fn convert_program(program: tacky::Program) -> asm::Program {
    let tacky::Program::Program(func) = program;
    asm::Program::Program(convert_function(func))
}

fn convert_function(func: tacky::Function) -> asm::Function {
    let mut instructions = Vec::new();
    for instr in func.body {
        convert_instruction(instr, &mut instructions);
    }
    asm::Function { name: func.name, instructions }
}

fn convert_instruction(instr: tacky::Instruction, out: &mut Vec<asm::Instruction>) {
    match instr {
        tacky::Instruction::Return(val) => {
            out.push(asm::Instruction::Mov(
                convert_val(val),
                asm::Operand::Reg(asm::Reg::RV),
            ));
            out.push(asm::Instruction::Ret);
        }
        tacky::Instruction::Unary(op, src, dst) => {
            let asm_dst = convert_val(dst);
            out.push(asm::Instruction::Mov(convert_val(src), asm_dst.clone()));
            out.push(asm::Instruction::Unary(convert_unop(op), asm_dst));
        }
    }
}

fn convert_val(val: tacky::Val) -> asm::Operand {
    match val {
        tacky::Val::Constant(n) => asm::Operand::Imm(n),
        tacky::Val::Var(name) => asm::Operand::Pseudo(name),
    }
}

fn convert_unop(op: tacky::UnaryOperator) -> asm::UnaryOperator {
    match op {
        tacky::UnaryOperator::Complement => asm::UnaryOperator::Not,
        tacky::UnaryOperator::Negate => asm::UnaryOperator::Neg,
    }
}

// Pass 2: Replace each Pseudo operand with a Stack slot

struct PseudoMap {
    map: HashMap<String, i64>,
    next_offset: i64,
}

impl PseudoMap {
    fn new() -> Self {
        Self { map: HashMap::new(), next_offset: 0 }
    }

    fn lookup(&mut self, name: &str) -> i64 {
        if let Some(&offset) = self.map.get(name) {
            return offset;
        }
        self.next_offset -= 4;
        self.map.insert(name.to_string(), self.next_offset);
        self.next_offset
    }

    fn stack_size(&self) -> i64 {
        -self.next_offset
    }
}

fn replace_pseudos(program: asm::Program) -> (asm::Program, i64) {
    let asm::Program::Program(mut func) = program;
    let mut pseudos = PseudoMap::new();
    for instr in &mut func.instructions {
        replace_in_instruction(instr, &mut pseudos);
    }
    let stack_size = pseudos.stack_size();
    (asm::Program::Program(func), stack_size)
}

fn replace_in_instruction(instr: &mut asm::Instruction, pseudos: &mut PseudoMap) {
    match instr {
        asm::Instruction::Mov(src, dst) => {
            replace_in_operand(src, pseudos);
            replace_in_operand(dst, pseudos);
        }
        asm::Instruction::Unary(_, op) => {
            replace_in_operand(op, pseudos);
        }
        asm::Instruction::AllocateStack(_) | asm::Instruction::Ret => {}
    }
}

fn replace_in_operand(op: &mut asm::Operand, pseudos: &mut PseudoMap) {
    if let asm::Operand::Pseudo(name) = op {
        let offset = pseudos.lookup(name);
        *op = asm::Operand::Stack(offset);
    }
}

// Pass 3: Insert AllocateStack and rewrite Mov with two Stack operands

fn fix_instructions(program: asm::Program, stack_size: i64) -> asm::Program {
    let asm::Program::Program(mut func) = program;
    let old = std::mem::take(&mut func.instructions);
    let mut new_instructions = Vec::with_capacity(old.len() + 1);
    if stack_size > 0 {
        new_instructions.push(asm::Instruction::AllocateStack(stack_size));
    }
    for instr in old {
        fix_instruction(instr, &mut new_instructions);
    }
    func.instructions = new_instructions;
    asm::Program::Program(func)
}

fn fix_instruction(instr: asm::Instruction, out: &mut Vec<asm::Instruction>) {
    match instr {
        asm::Instruction::Mov(asm::Operand::Stack(src), asm::Operand::Stack(dst)) => {
            out.push(asm::Instruction::Mov(
                asm::Operand::Stack(src),
                asm::Operand::Reg(asm::Reg::Scratch1),
            ));
            out.push(asm::Instruction::Mov(
                asm::Operand::Reg(asm::Reg::Scratch1),
                asm::Operand::Stack(dst),
            ));
        }
        other => out.push(other),
    }
}
