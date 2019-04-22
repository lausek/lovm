use super::*;

use std::collections::HashMap;

type Set<T> = HashMap<T, ()>;
pub type Function = CodeBlock;

pub struct FunctionBuilder {
    args: Vec<Name>,
    consts: HashMap<Name, Value>,
    locals: Set<Name>,
    globals: Set<Name>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            args: vec![],
            consts: HashMap::new(),
            locals: Set::new(),
            globals: Set::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<Name>) -> Self {
        self.args = args;
        for arg in self.args.iter() {
            self.locals.insert(arg.clone(), ());
        }
        self
    }

    pub fn build(mut self) -> Result<Function, ()> {
        Err(())
    }
}
