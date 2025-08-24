use std::process::Command;

use crate::compilation_error::{CompilationError, DownstreamCompileFailed};

pub fn downstream_compile(
    file: &String,
    output_name: &String,
) -> Option<Box<dyn CompilationError>> {
    let exc = Command::new("gcc")
        .arg(file.clone())
        .arg("-o")
        .arg(output_name)
        .status();

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
