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

pub type CompileResult = Result<Unit, Error>;
pub type Macro = Box<fn(&mut Unit) -> Result<(), Error>>;

const fn mkref(raw: usize) -> Code {
    Code::Value(Value::Ref(raw))
}

// if a label lookup doesn't deliver a result while generating, remember the labels
// name and the current generation offset for later. after all generation is done, we will
// go for a final lookup and insert the now existing result at the index on the codeblock.
#[derive(Clone, Debug)]
pub enum LabelOffset {
    // the label already occurred while compiling the program; this contains
    // its offset inside the codeblock
    Resolved(usize),
    // the label is still unknown. contains a list of indices where we have
    // to insert the resolved index
    Unresolved(Vec<(Ident, usize)>),
}

pub struct Compiler {
    macs: HashMap<&'static str, Macro>,
    unit: Option<Unit>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            macs: default_macros(),
            unit: None,
        }
    }

    pub fn compile(&mut self, src: &str) -> CompileResult {
        self.unit = Some(Unit::from(src.to_string()));

        {
            let unit = self.unit.as_mut().unwrap();
            let ast = parser::parse(&unit.src)?;

            for step in ast.into_iter() {
                match step {
                    Ast::Declare(value) => unit.declare_value(value)?,
                    Ast::Label(ident) => unit.declare_label(ident, unit.codeblock.len())?,
                    Ast::Macro(ident) => match self.macs.get(&ident.raw.as_ref()) {
                        Some(mac) => mac(unit)?,
                        _ => unreachable!(),
                    },
                    Ast::Statement(stmt) => unit.compile_statement(stmt)?,
                }
            }

            self.check_resolved()?;
        }

        let unit = self.unit.take().unwrap();

        Ok(unit)
    }

    fn check_resolved(&self) -> Result<(), Error> {
        let unit = self.unit.as_ref().unwrap();
        let mut errs = vec![];

        for (_, off) in unit.labels.iter() {
            match off {
                LabelOffset::Resolved(_) => {}
                LabelOffset::Unresolved(positions) => {
                    for (ident, _) in positions.iter() {
                        errs.push(raise::not_declared::<CompileResult>(ident).err().unwrap());
                    }
                }
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.into())
        }
    }
}
