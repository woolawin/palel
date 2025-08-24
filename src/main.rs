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

use parser::parse;
use std::process;
use transpiler_c::transpile;

use crate::build_task::{BuildTask, load};
use crate::core::Of;
use crate::palel::Src;
use crate::renderer_c::render;
use crate::toolkit_c::CToolKit;

fn main() {
    let mut task = BuildTask::default();
    if let Some(err) = load(&mut task) {
        print!("{}", err.message());
        process::exit(err.exit_code());
    }

    let mut src = Src::default();
    for file in &task.src_files {
        if let Some(err) = parse(&mut src, &file) {
            print!("{}", err.message());
            process::exit(err.exit_code());
        }
    }
    let toolkit = CToolKit {};
    let result = match transpile(&src, &toolkit) {
        Of::Ok(tp) => tp,
        Of::Error(err) => {
            println!("{}", err.message());
            process::exit(err.exit_code());
        }
    };

    println!("{}", render(&result));
}
