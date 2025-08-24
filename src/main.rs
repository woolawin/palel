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

use crate::build_task::BuildTask;
fn main() {
    let mut task = BuildTask::default();
    if let Some(err) = task.run() {
        print!("{}", err.message());
        process::exit(err.exit_code());
    }
}
