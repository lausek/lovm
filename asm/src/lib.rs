#![feature(bind_by_move_pattern_guards)]

pub mod compiler;

pub use self::compiler::*;

pub use lovm::code::*;

pub fn into_program(unit: Unit) -> Program {
    let mut program = Program::with_code(unit.codeblock);

    *program.labels_mut() = unit
        .labels
        .iter()
        .map(|(ident, off)| match off {
            LabelOffset::Resolved(off) => (ident.raw.clone(), *off),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    program
}
