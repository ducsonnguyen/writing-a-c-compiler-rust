use std::fmt;

pub enum Program {
    Program(Function),
}

pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOperator, Operand),
    AllocateStack(i64),
    Ret,
}

pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Clone)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(String),
    Stack(i64),
}

#[derive(Clone, Copy)]
pub enum Reg {
    RV,
    Scratch1,
}

fn indent(s: &str) -> String {
    s.lines().map(|l| format!("    {l}")).collect::<Vec<_>>().join("\n")
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Program::Program(func) = self;
        write!(f, "Program(\n{}\n)", indent(&func.to_string()))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instructions = self
            .instructions
            .iter()
            .map(|i| format!("    {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "Function(\n    name: {:?}\n    instructions:\n{}\n)", self.name, instructions)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Mov(src, dst) => write!(f, "Mov({src}, {dst})"),
            Instruction::Unary(op, operand) => write!(f, "Unary({op}, {operand})"),
            Instruction::AllocateStack(n) => write!(f, "AllocateStack({n})"),
            Instruction::Ret => write!(f, "Ret"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Neg => write!(f, "Neg"),
            UnaryOperator::Not => write!(f, "Not"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Imm(n) => write!(f, "Imm({n})"),
            Operand::Reg(r) => write!(f, "Reg({r})"),
            Operand::Pseudo(name) => write!(f, "Pseudo({name:?})"),
            Operand::Stack(n) => write!(f, "Stack({n})"),
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reg::RV => write!(f, "RV"),
            Reg::Scratch1 => write!(f, "Scratch1"),
        }
    }
}
