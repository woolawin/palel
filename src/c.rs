#[derive(Debug, PartialEq)]
pub struct CSrc {
    pub includes: Vec<CInclude>,
    pub functions: Vec<CFunction>,
}

#[derive(Debug, PartialEq, Default)]
pub struct CSrcPatch {
    pub includes: Vec<CInclude>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CInclude {
    pub file: String,
}

#[derive(Debug, PartialEq)]
pub struct CFunction {
    pub name: String,
    pub return_type: CType,
    pub block: CBlock,
}

#[derive(Debug, PartialEq)]
pub struct CType {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CBlock {
    pub statements: Vec<CStatement>,
}

#[derive(Debug, PartialEq)]
pub enum CStatement {
    FunctionCall(CFunctionCall),
    Return(CReturn),
    Variable(CVariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct CReturn {
    pub value: Option<CExpression>,
}

impl CReturn {
    pub fn to_statement(self) -> CStatement {
        CStatement::Return(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct CFunctionCall {
    pub function_name: String,
    pub arguments: Vec<CExpression>,
}

impl CFunctionCall {
    pub fn to_statement(self) -> CStatement {
        CStatement::FunctionCall(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum CLiteral {
    String(String),
    Number(String),
}

impl CLiteral {
    pub fn to_expression(self) -> CExpression {
        CExpression::Literal(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum CExpression {
    Literal(CLiteral),
}

#[derive(Debug, PartialEq)]
pub struct CVariableDeclaration {
    pub name: String,
    pub var_type: CType,
    pub is_pointer: bool,
    pub value: CExpression,
}

impl CVariableDeclaration {
    pub fn to_statement(self) -> CStatement {
        CStatement::Variable(self)
    }
}
