use crate::palel::{SchemaType, Type};

const NOOP_ERROR: i32 = 1;
const DISK_ERROR: i32 = 2;
const PARSE_ERROR: i32 = 3;
const LOGIC_ERROR: i32 = 4;
const TYPE_ERROR: i32 = 5;
const TRANSPILE_ERROR: i32 = 20;
const DOWNSTREAM_ERROR: i32 = 21;

pub trait CompilationError {
    fn message(&self) -> String;
    fn exit_code(&self) -> i32 {
        TRANSPILE_ERROR
    }
}

#[derive(Debug, PartialEq)]
pub struct UnknownInterface {
    pub interface: String,
}

impl CompilationError for UnknownInterface {
    fn message(&self) -> String {
        format!("could not find interface '{}'", self.interface)
    }
}

pub struct NoSourceFiles {
    pub dir: String,
}

impl CompilationError for NoSourceFiles {
    fn message(&self) -> String {
        format!("no palel source files were found in {}", self.dir)
    }

    fn exit_code(&self) -> i32 {
        NOOP_ERROR
    }
}

pub struct FailedToReadSrcFile {
    pub file: String,
}

impl CompilationError for FailedToReadSrcFile {
    fn message(&self) -> String {
        format!("failed to read source file '{}'", self.file)
    }

    fn exit_code(&self) -> i32 {
        DISK_ERROR
    }
}

pub struct FailedToWriteToFile {
    pub file: String,
}

impl CompilationError for FailedToWriteToFile {
    fn message(&self) -> String {
        format!("failed to write to file '{}'", self.file)
    }

    fn exit_code(&self) -> i32 {
        DISK_ERROR
    }
}

pub struct FailedToParseSrcFile {
    pub file: String,
}

impl CompilationError for FailedToParseSrcFile {
    fn message(&self) -> String {
        format!("failed to parse source file '{}'", self.file)
    }

    fn exit_code(&self) -> i32 {
        PARSE_ERROR
    }
}

pub struct DownstreamCompileFailed {}

impl CompilationError for DownstreamCompileFailed {
    fn message(&self) -> String {
        format!("downstream compiler failed")
    }

    fn exit_code(&self) -> i32 {
        DOWNSTREAM_ERROR
    }
}

pub struct VariableTypeAmbiguous {}

impl CompilationError for VariableTypeAmbiguous {
    fn message(&self) -> String {
        format!("could not determine type of variable")
    }

    fn exit_code(&self) -> i32 {
        LOGIC_ERROR
    }
}

pub struct CouldNotTranspileType {}

impl CompilationError for CouldNotTranspileType {
    fn message(&self) -> String {
        format!("could not transpile type")
    }

    fn exit_code(&self) -> i32 {
        TRANSPILE_ERROR
    }
}

pub struct IncompatibleTypes {
    pub expected: Type,
    pub actual: Type,
}

impl CompilationError for IncompatibleTypes {
    fn message(&self) -> String {
        format!(
            "incompatible types, expected {}, received {}",
            self.expected.to_string(),
            self.actual.to_string()
        )
    }

    fn exit_code(&self) -> i32 {
        TYPE_ERROR
    }
}

pub struct TypeNotNullable {
    pub received_type: Type,
}

impl CompilationError for TypeNotNullable {
    fn message(&self) -> String {
        format!("type {} is not nullable", self.received_type.to_string())
    }
    fn exit_code(&self) -> i32 {
        TYPE_ERROR
    }
}
