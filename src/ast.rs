use std::fmt;

pub enum Program {
    Program(Function),
}

pub struct Function {
    pub name: String,
    pub body: Statement,
}

pub enum Statement {
    Return(Exp),
}

pub enum Exp {
    Constant(i64),
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
        write!(
            f,
            "Function(\n    name: {:?}\n    body: {}\n)",
            self.name, self.body
        )
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Statement::Return(exp) = self;
        write!(f, "Return({})", exp)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Exp::Constant(n) = self;
        write!(f, "Constant({})", n)
    }
}
