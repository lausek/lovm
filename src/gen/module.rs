use super::*;

// ---- example
// pseudocode:
//      foo(x, y): ...
//      bar(x):    ...
//
// rust
//      let foo = gen::FunctionBuilder::new()...;
//      let bar = gen::FunctionBuilder::new()...;
//      gen::ModuleBuilder::new()
//          .decl("foo", foo)
//          .decl("bar", bar)
//          .build()
//
// ---- explanation

#[derive(Clone, Debug)]
pub struct ModuleBuilder {
    slots: Vec<(Name, CodeObject)>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self { slots: vec![] }
    }

    pub fn from_object(co: CodeObject) -> Self {
        Self {
            slots: vec![("main".to_string(), co)],
        }
    }

    pub fn decl<T>(&mut self, name: T, co: CodeObject) -> &mut Self
    where
        T: std::string::ToString,
    {
        self.slots.push((name.to_string(), co));
        self
    }

    pub fn build(self) -> BuildResult<Module> {
        let mut module = Module::new();
        module.inner = self.slots;
        Ok(module)
    }
}
