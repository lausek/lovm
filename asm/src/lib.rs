#![feature(bind_by_move_pattern_guards)]

// TODO: implement local labels (prefix char _, eg _inner)
// DONE: implement lexing for SoftPunct (alias for combined whitespace; allows for prefix check if w/o SoftPunct)
// DONE: implement macros (prefix char ., eg .skip)

pub mod compiler;

pub use self::compiler::*;

pub use lovm::code::*;

use std::fs;
use std::io::Read;

pub fn compile_file(path: &str) -> Result<Unit, Error> {
    println!("{}", path);
    if let Ok(mut in_file) = fs::File::open(path) {
        let mut src = String::new();
        in_file
            .read_to_string(&mut src)
            .expect("cannot read program file");

        let mut compiler = compiler::Compiler::new();
        compiler.compile_path(src.as_ref(), path.to_string())?;
        compiler.finish()
    } else {
        Err(Error::new().msg("cannot open program file".to_string()))
    }
}

pub fn into_program(unit: Unit) -> Program {
    let mut program = Program::with_code(unit.code);

    // TODO: this doesn't work anymore due to the new memory management concept. waiting for
    // generator library...
    /*
        *program.labels_mut() = unit
            .labels
            .iter()
            .filter_map(|(ident, label)| match label.is_exported() {
                true => Some((ident.raw.clone(), label.decl.as_ref().unwrap().1)),
                _ => None,
            })
            .collect::<Vec<_>>();
    */

    program
}
