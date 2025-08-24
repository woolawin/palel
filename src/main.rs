mod build_task;
mod c;
mod compilation_error;
mod core;
mod palel;
mod parser;
mod renderer_c;
mod toolkit_c;
mod transpiler_c;
mod transpiler_c_patch;

use std::process;

use crate::build_task::{BuildTask, run};

fn main() {
    let mut task = BuildTask::default();
    if let Some(err) = run(&mut task) {
        print!("{}", err.message());
        process::exit(err.exit_code());
    }
}
