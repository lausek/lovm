use super::*;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Unit {
    pub(crate) codeblock: CodeBlock,
    pub(crate) labels: HashMap<Ident, LabelOffset>,
    pub(crate) src: String,
}

impl Unit {
    pub fn from(src: String) -> Self {
        Self {
            codeblock: CodeBlock::new(),
            labels: HashMap::new(),
            src,
        }
    }

    pub fn push_inx(&mut self, inx: Instruction) {
        self.codeblock.push(Code::Instruction(inx));
    }

    pub fn compile_operand(&mut self, op: Operand) -> Result<(), String> {
        // we have to push a placeholder value or the index will become corrupt
        let mut code = mkref(std::usize::MAX);

        match op {
            Operand::Ident(ident) => match self.labels.get_mut(&ident) {
                Some(LabelOffset::Resolved(off)) => code = mkref(*off),
                Some(LabelOffset::Unresolved(positions)) => {
                    positions.push((ident.clone(), self.codeblock.len()))
                }
                _ => {
                    self.labels.insert(
                        ident.clone(),
                        LabelOffset::Unresolved(vec![(ident.clone(), self.codeblock.len())]),
                    );
                }
            },
            Operand::Register(reg) => code = Code::Register(reg),
            Operand::Value(value) => code = Code::Value(value),
            Operand::Str(s) => {
                // TODO: write s as bytes in consequtive order to memory
                // TODO: insert reference to string pool here
                for c in s.bytes() {
                    self.codeblock.push(Code::Value(Value::I(c as i8)));
                }
                return Ok(());
            }
            Operand::Deref(_) => unreachable!(),
        }

        self.codeblock.push(code);
        Ok(())
    }

    pub fn declare_label(&mut self, label: Ident, off: usize) -> Result<(), Error> {
        match self
            .labels
            .insert(label.clone(), LabelOffset::Resolved(off))
        {
            Some(LabelOffset::Resolved(_)) => raise::redeclared(&label),
            // use reverse order to not invalidate indices
            Some(LabelOffset::Unresolved(positions)) => {
                for (_, pos) in positions.into_iter().rev() {
                    *self.codeblock.get_mut(pos).unwrap() = mkref(off);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn declare_value(&mut self, value: String) -> Result<(), String> {
        let value = Value::from_str(&value)?;
        self.codeblock.push(Code::Value(value));
        Ok(())
    }
}
