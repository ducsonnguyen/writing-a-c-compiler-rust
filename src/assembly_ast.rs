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
    Ret,
}

pub enum Operand {
    Imm(i64),
    Register,
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
            Instruction::Ret => write!(f, "Ret"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Imm(n) => write!(f, "Imm({n})"),
            Operand::Register => write!(f, "Register"),
        }
    }
}
