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
    pub identifier: SchemaIdentifier,
    pub postfix: TypePostfix,
    pub family: TypeFamily,
    pub size: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaIdentifier {
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    Char,
    UserDefined(String),
}

impl SchemaIdentifier {
    pub fn family(&self) -> TypeFamily {
        match self {
            SchemaIdentifier::Int32 | SchemaIdentifier::Int64 => TypeFamily::Int,
            SchemaIdentifier::Float32 | SchemaIdentifier::Float64 => TypeFamily::Float,
            _ => TypeFamily::None,
        }
    }

    pub fn size(&self) -> Option<i32> {
        match self {
            SchemaIdentifier::Int32 | SchemaIdentifier::Float32 => Some(32),
            SchemaIdentifier::Int64 | SchemaIdentifier::Float64 => Some(64),
            _ => None,
        }
    }
}

impl ToString for SchemaIdentifier {
    fn to_string(&self) -> String {
        match self {
            Self::Int32 => "Int32".to_string(),
            Self::Int64 => "Int64".to_string(),
            Self::Float32 => "Float32".to_string(),
            Self::Float64 => "Float64".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::Char => "Char".to_string(),
            Self::UserDefined(id) => id.clone(),
        }
    }
}

impl SchemaType {
    pub fn set_identifier(&mut self, new_identifier: SchemaIdentifier) {
        self.family = new_identifier.family();
        self.size = new_identifier.size();
        self.identifier = new_identifier;
    }
}

pub fn schema_identifier_from_string(value: String) -> SchemaIdentifier {
    match value.as_str() {
        "Int32" => SchemaIdentifier::Int32,
        "Int64" => SchemaIdentifier::Int64,
        "Float32" => SchemaIdentifier::Float32,
        "Float64" => SchemaIdentifier::Float64,
        "Bool" => SchemaIdentifier::Bool,
        "Char" => SchemaIdentifier::Char,
        _ => SchemaIdentifier::UserDefined(value.clone()),
    }
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(self.identifier.to_string().as_str());
        match self.postfix {
            TypePostfix::Opt => output.push_str("?"),
            TypePostfix::Err => output.push_str("!"),
            _ => {}
        };
        output
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    Addr(Option<SchemaType>),
    Ref(SchemaType),
    Dim(SchemaType),
    Null,
}

impl ExpressionType {
    pub fn to_type(self) -> Option<Type> {
        match self {
            ExpressionType::Addr(t) => Some(Type::Addr(t)),
            ExpressionType::Ref(t) => Some(Type::Ref(t)),
            ExpressionType::Dim(t) => Some(Type::Dim(t)),
            ExpressionType::Null => None,
        }
    }
}

impl ToString for ExpressionType {
    fn to_string(&self) -> String {
        let mut output = String::new();
        match self {
            ExpressionType::Addr(typ) => {
                output.push_str("addr ");
                if let Some(addrtyp) = typ {
                    output.push_str(addrtyp.to_string().as_str());
                }
            }
            ExpressionType::Ref(reftype) => {
                output.push_str("ref ");
                output.push_str(reftype.to_string().as_str());
            }
            ExpressionType::Dim(dimtype) => {
                output.push_str("dim ");
                output.push_str(dimtype.to_string().as_str());
            }
            ExpressionType::Null => {
                output.push_str("null");
            }
        }
        output
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Addr(Option<SchemaType>),
    Ref(SchemaType),
    Dim(SchemaType),
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
        }
        output
    }
}

pub fn schema_type(identifier: SchemaIdentifier) -> SchemaType {
    let family = identifier.family();
    let size = identifier.size();
    SchemaType {
        identifier: identifier,
        postfix: TypePostfix::None,
        family: family,
        size: size,
    }
}

pub fn charseq_type() -> SchemaType {
    SchemaType {
        identifier: SchemaIdentifier::Char,
        postfix: TypePostfix::None,
        family: TypeFamily::None,
        size: None,
    }
}
