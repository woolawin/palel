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
use std::fs;
use std::io;
use transpiler_c::transpile;

use crate::toolkit_c::CToolKit;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("main.pl")?;
    let toolkit = CToolKit {};
    if let Ok(src) = parse(&contents) {
        _ = transpile(&src, &toolkit);
    }
    return Ok(());
}
