#[derive(Debug, PartialEq)]
pub struct Src {
    pub programs: Vec<Program>,
}

impl Default for Src {
    fn default() -> Src {
        Src { programs: vec![] }
    }
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
    Return(Return),
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ProcedureCall {
    pub interface: String,
    pub identifier: String,
    pub arguments: Vec<Expression>,
}

impl ProcedureCall {
    pub fn to_statement(self) -> Statement {
        return Statement::ProcedureCall(self);
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(String),
    Null,
}

impl Literal {
    pub fn to_expression(self) -> Expression {
        Expression::Literal(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
}
