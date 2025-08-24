mod build_task;
mod c;
mod compilation_error;
mod core;
mod downstream_compiler_c;
mod palel;
mod parser;
mod renderer_c;
mod toolkit_c;
mod transpiler_c;
mod transpiler_c_patch;

use std::process;

use crate::build_task::{create_build_task, default_build_task_config, run_build_task};

fn main() {
    let config = default_build_task_config();
    let mut task = create_build_task(config);
    if let Some(err) = run_build_task(&mut task) {
        print!("{}", err.message());
        process::exit(err.exit_code());
    }
}
