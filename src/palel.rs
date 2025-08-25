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
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Option<Expression>,
}

impl Return {
    pub fn to_statement(self) -> Statement {
        Statement::Return(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct ProcedureCall {
    pub interface: String,
    pub identifier: String,
    pub arguments: Vec<Expression>,
}

impl ProcedureCall {
    pub fn to_statement(self) -> Statement {
        Statement::ProcedureCall(self)
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

#[derive(Debug, PartialEq)]
pub enum MemoryModifier {
    Dim,
    Var,
    Ref,
    Addr,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub memory: MemoryModifier,
    pub identifier: String,
    pub value_type: Option<Type>,
    pub value: Expression,
}

impl VariableDeclaration {
    pub fn to_statement(self) -> Statement {
        Statement::Variable(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum TypePostfix {
    Opt,
    Err,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub identifier: String,
    pub postfix: TypePostfix,
}
