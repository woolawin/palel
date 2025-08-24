const NOOP_ERROR: i32 = 1;
const DISK_ERROR: i32 = 2;
const PARSE_ERROR: i32 = 3;
const TRANSPILE_ERROR: i32 = 4;

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
