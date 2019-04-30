use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Modules(Vec<Module>);

impl Modules {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn lookup(&self, name: &Name) -> Option<&CodeObject> {
        for module in self.0.iter() {
            if let Some(co) = module.get(name) {
                return Some(co);
            }
        }
        None
    }

    pub fn load(&mut self, module: &Module) -> Result<(), String> {
        self.0.push(module.clone());
        Ok(())
    }
}
