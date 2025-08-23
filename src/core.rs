use crate::compilation_error::CompilationError;

pub enum Of<T> {
    Ok(T),
    Error(Box<dyn CompilationError>),
}

impl<T> Of<T> {
    pub fn unwrap(self) -> T {
        match self {
            Of::Ok(value) => value,
            Of::Error(err) => panic!("called `Of::unwrap()` on an `Error`"),
        }
    }
}
