use crate::compilation_error::CompilationError;

pub enum Of<T> {
    Ok(T),
    Error(Box<dyn CompilationError>),
}
