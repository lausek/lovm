use super::*;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub space: Space,
    pub inner: Vec<(Name, CodeObjectRef)>,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            space: Space::new(),
            inner: vec![],
        }
    }

    // TODO: rename to `lookup` or something like that
    pub fn get(&self, name: &Name) -> Option<CodeObjectRef> {
        for (sname, co) in self.inner.iter() {
            if sname == name {
                return Some(co.clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: &Name, co: CodeObject) {
        if let Some(mut slot) = self.get(name) {
            *slot.get_mut() = co;
        } else {
            self.inner.push((name.clone(), CodeObjectRef::from(co)));
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn with_code(code: CodeBlock) -> Self {
        let mut new = Self::new();
        let mut co = CodeObject::new();
        co.inner = code;
        new.set(&"main".into(), co);
        new
    }

    pub fn code(&self) -> CodeObjectRef {
        self.inner
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, code)| code.clone())
            .unwrap()
    }

    pub fn slots(&self) -> &Vec<(Name, CodeObjectRef)> {
        &self.inner
    }

    pub fn slots_mut(&mut self) -> &mut Vec<(Name, CodeObjectRef)> {
        &mut self.inner
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unit(slots: {})", self.inner.len())?;
        for (name, co) in self.inner.iter() {
            let co: &CodeObject = co.borrow();
            writeln!(f, "\t{}({}): {:?}, {:?}", name, co.argc, co.space, co.inner)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unit(slots: {})", self.inner.len())?;
        for (name, co) in self.inner.iter() {
            writeln!(f, "\t{}: {}", name, co)?;
        }
        Ok(())
    }
}
