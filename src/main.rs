mod c;
mod palel;
mod parser;
mod renderer_c;
mod transpiler_c;

use parser::parse;
use std::fs;
use std::io;
use transpiler_c::transpile;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("main.pl")?;
    if let Ok(src) = parse(&contents) {
        _ = transpile(&src);
    }
    return Ok(());
}
