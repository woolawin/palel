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
    Boolean(bool),
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

#[derive(Debug, PartialEq, Clone)]
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
    pub schema_type: Option<SchemaType>,
    pub expression: Expression,
}

impl VariableDeclaration {
    pub fn to_statement(self) -> Statement {
        Statement::Variable(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypePostfix {
    Opt,
    Err,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeFamily {
    Int,
    Float,
    None,
}

pub fn type_family_of(type_name: &String) -> TypeFamily {
    match type_name.as_str() {
        "Int32" | "Int64" => TypeFamily::Int,
        "Float32" | "Float64" => TypeFamily::Float,
        _ => TypeFamily::None,
    }
}

pub fn type_size_of(type_name: &String) -> Option<i32> {
    match type_name.as_str() {
        "Int32" | "Float32" => Some(32),
        "Int64" | "Float64" => Some(64),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SchemaType {
    pub identifier: String,
    pub postfix: TypePostfix,
    pub family: TypeFamily,
    pub size: Option<i32>,
}

impl SchemaType {
    pub fn set_identifier(&mut self, new_identifier: String) {
        self.family = type_family_of(&new_identifier);
        self.size = type_size_of(&new_identifier);
        self.identifier = new_identifier;
    }
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(self.identifier.as_str());
        match self.postfix {
            TypePostfix::Opt => output.push_str("?"),
            TypePostfix::Err => output.push_str("!"),
            _ => {}
        };
        output
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Addr(Option<SchemaType>),
    Ref(SchemaType),
    Dim(SchemaType),
    Null,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        let mut output = String::new();
        match self {
            Type::Addr(typ) => {
                output.push_str("addr ");
                if let Some(addrtyp) = typ {
                    output.push_str(addrtyp.to_string().as_str());
                }
            }
            Type::Ref(reftype) => {
                output.push_str("ref ");
                output.push_str(reftype.to_string().as_str());
            }
            Type::Dim(dimtype) => {
                output.push_str("dim ");
                output.push_str(dimtype.to_string().as_str());
            }
            Type::Null => {
                output.push_str("null");
            }
        }
        output
    }
}

pub fn null_type() -> SchemaType {
    SchemaType {
        identifier: "Null".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::None,
        size: None,
    }
}

pub fn int32_type() -> SchemaType {
    SchemaType {
        identifier: "Int32".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::Int,
        size: Some(32),
    }
}

pub fn int64_type() -> SchemaType {
    SchemaType {
        identifier: "Int64".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::Int,
        size: Some(64),
    }
}

pub fn float32_type() -> SchemaType {
    SchemaType {
        identifier: "Float32".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::Float,
        size: Some(32),
    }
}

pub fn float64_type() -> SchemaType {
    SchemaType {
        identifier: "Float64".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::Float,
        size: Some(64),
    }
}

pub fn charseq_type() -> SchemaType {
    SchemaType {
        identifier: "CharSeq".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::None,
        size: None,
    }
}

pub fn bool_type() -> SchemaType {
    SchemaType {
        identifier: "Bool".to_string(),
        postfix: TypePostfix::None,
        family: TypeFamily::None,
        size: None,
    }
}
