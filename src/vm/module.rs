use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Units(pub Vec<Unit>);

impl Units {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn lookup(&self, name: &Name) -> Option<&CodeObject> {
        for unit in self.0.iter() {
            if let Some(co) = unit.get(name) {
                return Some(co);
            }
        }
        None
    }

    pub fn load(&mut self, unit: &Unit) -> Result<(), String> {
        self.0.push(unit.clone());
        Ok(())
    }
}
