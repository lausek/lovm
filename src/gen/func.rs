use super::*;

// ---- example
// pseudocode:
//      f(x, y):
//          z = x + y
//          return z
// rust
//      gen::FunctionBuilder::new()
//          .with_args(vec!["x", "y"])      // TODO: is it `args` or `params` here? there was a difference...
//          .step(gen::Op::Add, "x", "y")
//          .store("z")
//          .end()
//          .build()
//
// ---- explanation

pub type Function = CodeObject;

pub struct FunctionBuilder {
    args: Vec<Name>,
    consts: HashMap<Name, Value>,
    locals: Set<Name>,
    globals: Set<Name>,
    seq: Sequence,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            args: vec![],
            consts: HashMap::new(),
            locals: Set::new(),
            globals: Set::new(),
            seq: Sequence::new(),
        }
    }

    pub fn with_args<T>(mut self, args: Vec<T>) -> Self
    where
        T: std::string::ToString,
    {
        // TODO: optimize this
        self.args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
        for arg in self.args.iter() {
            self.locals.insert(arg.to_string(), ());
        }
        self
    }

    pub fn build(mut self) -> BuildResult<Function> {
        let mut func = Function::new();
        // TODO: compile `seq` into bytecode
        Ok(func)
    }
}
