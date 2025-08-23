pub trait CompilationError {}

#[derive(Debug, PartialEq)]
pub struct UnknownInterface {
    pub interface: String,
}

impl CompilationError for UnknownInterface {}
