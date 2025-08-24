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
    pub argument_list: Vec<Argument>,
}

impl ProcedureCall {
    pub fn to_statement(self) -> Statement {
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
    pub fn to_argument(self) -> Argument {
        Argument::Literal(self)
    }

    pub fn to_expression(self) -> Expression {
        Expression::Literal(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
}
