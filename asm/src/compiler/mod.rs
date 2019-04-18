pub use super::*;

pub mod error;
mod mac;
mod objects;
mod parser;
mod unit;

pub use self::error::*;
pub use self::mac::*;
pub use self::objects::*;
pub use self::parser::*;
pub use self::unit::*;

use lovm::value::Value;

use std::collections::HashMap;

// TODO: fix offsets of included programs
//       1. collect all static strings into a big section at program end
//       2. label resolvement must change to be totally lazily
//          - no LabelOffset::Resolved anymore
//       3. insert Str(_) or Ref(_) to resolve labels at end of compilation
// WIP: add `.export <name>` macro to decide which label should be exported

pub type CompileResult = Result<Unit, Error>;

const fn mkref(raw: usize) -> Code {
    Code::Value(Value::Ref(raw))
}

fn embed_string(s: &str, cb: &mut Vec<Code>) {
    for b in s.bytes() {
        cb.push(Code::Value(Value::I(b as i8)));
    }
    // null terminator
    cb.push(Code::Value(Value::I(0)));
}

pub struct Compiler {
    macs: MacroTable,
    unit: Option<Unit>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            macs: default_macros(),
            unit: None,
        }
    }

    pub fn finish(&mut self) -> CompileResult {
        let mut unit = self.unit.take().unwrap();
        unit.link()?;
        Ok(unit)
    }

    pub fn compile_path(&mut self, src: &str, path: String) -> Result<(), Error> {
        self.unit = Some(Unit::from_path(src.to_string(), path));
        self.compile()
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        let unit = self.unit.as_mut().unwrap();
        let ast = parser::parse(&unit.src)?;

        for step in ast.into_iter() {
            match step {
                Ast::Declare(value) => unit.declare_value(value)?,
                Ast::Label(ident) => unit.declare_label(ident, unit.codeblock.len())?,
                Ast::Macro(ident, args) => match self.macs.get(&ident.raw.as_ref()) {
                    Some(mac) => mac(unit, args)?,
                    _ => unreachable!(),
                },
                Ast::Statement(stmt) => unit.compile_statement(stmt)?,
            }
        }

        Ok(())
    }
}
