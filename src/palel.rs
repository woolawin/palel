#[derive(Debug, PartialEq)]
pub struct Src {
    pub programs: Vec<Program>,
}

#[derive(Debug, PartialEq)]
pub struct DoBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub do_block: DoBlock,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    ProcedureCall(ProcedureCall),
}

#[derive(Debug, PartialEq)]
pub struct ProcedureCall {
    pub interface: String,
    pub identifier: String,
    pub argument_list: Vec<Argument>,
}

impl ProcedureCall {
    pub fn as_statement(self) -> Statement {
        return Statement::ProcedureCall(self);
    }
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(String),
}

impl Literal {
    pub fn as_argument(self) -> Argument {
        Argument::Literal(self)
    }
}
