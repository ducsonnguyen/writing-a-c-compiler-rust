use crate::ast;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub enum Program {
    Program(Function),
}

pub struct Function {
    pub name: String,
    pub body: Vec<Instruction>,
}

pub enum Instruction {
    Return(Val),
    Unary(UnaryOperator, Val, Val), // op, src, dst
}

#[derive(Clone)]
pub enum Val {
    Constant(i64),
    Var(String),
}

pub enum UnaryOperator {
    Complement,
    Negate,
}

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn make_temporary() -> String {
    let n = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("tmp.{n}")
}

pub fn gen_program(program: ast::Program) -> Program {
    let ast::Program::Program(func) = program;
    Program::Program(gen_function(func))
}

fn gen_function(func: ast::Function) -> Function {
    let mut body = Vec::new();
    gen_statement(func.body, &mut body);
    Function { name: func.name, body }
}

fn gen_statement(stmt: ast::Statement, instructions: &mut Vec<Instruction>) {
    match stmt {
        ast::Statement::Return(exp) => {
            let val = emit_tacky(exp, instructions);
            instructions.push(Instruction::Return(val));
        }
    }
}

fn emit_tacky(exp: ast::Exp, instructions: &mut Vec<Instruction>) -> Val {
    match exp {
        ast::Exp::Constant(n) => Val::Constant(n),
        ast::Exp::Unary(op, inner) => {
            let src = emit_tacky(*inner, instructions);
            let dst = Val::Var(make_temporary());
            let tacky_op = convert_unop(op);
            instructions.push(Instruction::Unary(tacky_op, src, dst.clone()));
            dst
        }
    }
}

fn convert_unop(op: ast::UnaryOperator) -> UnaryOperator {
    match op {
        ast::UnaryOperator::Complement => UnaryOperator::Complement,
        ast::UnaryOperator::Negate => UnaryOperator::Negate,
    }
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
        let body = self
            .body
            .iter()
            .map(|i| format!("    {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "Function(\n    name: {:?}\n    body:\n{}\n)", self.name, body)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Return(v) => write!(f, "Return({v})"),
            Instruction::Unary(op, src, dst) => write!(f, "Unary({op}, {src}, {dst})"),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Constant(n) => write!(f, "Constant({n})"),
            Val::Var(name) => write!(f, "Var({name:?})"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Complement => write!(f, "Complement"),
            UnaryOperator::Negate => write!(f, "Negate"),
        }
    }
}
