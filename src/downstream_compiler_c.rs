use std::process::Command;

use crate::compilation_error::{CompilationError, DownstreamCompileFailed};

pub fn downstream_compile(file: &String) -> Option<Box<dyn CompilationError>> {
    let exc = Command::new("gcc").arg(file.clone()).status();

    let err = DownstreamCompileFailed {};

    let status = match exc {
        Err(_) => {
            return Some(Box::new(err));
        }
        Ok(value) => value,
    };

    if !status.success() {
        return Some(Box::new(err));
    }
    None
}
