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
    pub is_pointer: bool,
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
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct CVariableDeclaration {
    pub name: String,
    pub var_type: CType,
    pub value: CExpression,
}

impl CVariableDeclaration {
    pub fn to_statement(self) -> CStatement {
        CStatement::Variable(self)
    }
}

pub fn void_type(pointer: bool) -> CType {
    CType {
        name: "void".to_string(),
        is_pointer: pointer,
    }
}

pub fn int_type() -> CType {
    CType {
        name: "int".to_string(),
        is_pointer: false,
    }
}

pub fn long_type() -> CType {
    CType {
        name: "long".to_string(),
        is_pointer: false,
    }
}

pub fn float_type() -> CType {
    CType {
        name: "float".to_string(),
        is_pointer: false,
    }
}

pub fn double_type() -> CType {
    CType {
        name: "double".to_string(),
        is_pointer: false,
    }
}
