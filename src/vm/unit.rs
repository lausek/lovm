use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Units(pub Vec<UnitRef>, HashMap<Name, UnitRef>);

impl Units {
    pub fn new() -> Self {
        let mut new = Self(vec![], HashMap::new());

        // default type for objects
        new.0.push(UnitRef::from(Unit::new()));
        let last = new.0.last().unwrap().clone();
        new.1.insert("object".to_string(), last);

        new
    }

    pub fn lookup(&self, name: &Name) -> Option<CodeObjectRef> {
        for module in self.0.iter() {
            let module: &Unit = module.borrow();
            if let Some(co) = module.get(name) {
                return Some(co.clone());
            }
        }
        None
    }

    pub fn lookup_ty(&self, name: &Name) -> Option<UnitRef> {
        self.1.get(name).and_then(|item| Some(item.clone()))
    }

    pub fn load(&mut self, module: &Unit) -> Result<(), String> {
        self.0.push(UnitRef::from(module.clone()));
        Ok(())
    }

    pub fn load_ty(&mut self, module: &Unit, name: Name) -> Result<(), String> {
        self.0.push(UnitRef::from(module.clone()));
        let last = self.0.last().unwrap().clone();
        self.1.insert(name, last);
        Ok(())
    }
}
