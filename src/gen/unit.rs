use super::*;

#[derive(Clone, Debug)]
pub struct UnitBuilder {
    slots: Vec<(Name, CodeObject)>,
}

impl UnitBuilder {
    pub fn new() -> Self {
        Self { slots: vec![] }
    }

    pub fn from_object(co: CodeObject) -> Self {
        Self {
            slots: vec![("main".to_string(), co)],
        }
    }

    pub fn set<T>(&mut self, name: T, co: CodeObject) -> &mut Self
    where
        T: std::string::ToString,
    {
        let sname = name.to_string();
        match self.slots.iter_mut().find(|slot| slot.0 == sname) {
            Some(slot) => slot.1 = co,
            _ => self.slots.push((name.to_string(), co)),
        }
        self
    }

    pub fn decl<T>(&mut self, name: T, co: CodeObject) -> &mut Self
    where
        T: std::string::ToString,
    {
        self.slots.push((name.to_string(), co));
        self
    }

    pub fn build(&self) -> BuildResult<Unit> {
        let mut unit = Unit::new();
        for (name, co) in self.slots.iter() {
            unit.code
                .push((name.clone(), CodeObjectRef::from(co.clone())));
        }
        Ok(unit)
    }
}
