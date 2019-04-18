#![feature(bind_by_move_pattern_guards)]

// TODO: implement local labels (prefix char _, eg _inner)
// TODO: implement lexing for SoftPunct (alias for combined whitespace; allows for prefix check if wo\ SoftPunct)
// DONE: implement macros (prefix char ., eg .skip)

pub mod compiler;

pub use self::compiler::*;

pub use lovm::code::*;

use std::fs;
use std::io::Read;

pub fn compile_file(path: &str) -> Result<Program, Error> {
    println!("{}", path);
    if let Ok(mut in_file) = fs::File::open(path) {
        let mut src = String::new();
        in_file
            .read_to_string(&mut src)
            .expect("cannot read program file");

        compiler::Compiler::new()
            .compile_path(src.as_ref(), path.to_string())
            .map(|unit| into_program(unit))
    } else {
        Err(Error::new().msg("cannot open program file".to_string()))
    }
}

pub fn into_program(unit: Unit) -> Program {
    let mut program = Program::with_code(unit.codeblock);

    *program.labels_mut() = unit
        .labels
        .iter()
        .filter_map(|(ident, label)| match label.is_exported() {
            true => Some((ident.raw.clone(), label.decl.as_ref().unwrap().1)),
            _ => None,
        })
        .collect::<Vec<_>>();

    program
}
