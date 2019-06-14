use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Units(pub Vec<Unit>);

impl Units {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn lookup(&self, name: &Name) -> Option<CodeObjectRef> {
        for module in self.0.iter() {
            if let Some(co) = module.get(name) {
                return Some(co.clone());
            }
        }
        None
    }

    pub fn load(&mut self, module: &Unit) -> Result<(), String> {
        self.0.push(module.clone());
        Ok(())
    }
}
