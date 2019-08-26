use super::*;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub space: Space,
    pub code: Vec<(Name, CodeObjectRef)>,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            space: Space::new(),
            code: vec![],
        }
    }

    // TODO: rename to `lookup` or something like that
    pub fn get(&self, name: &Name) -> Option<CodeObjectRef> {
        for (sname, co) in self.code.iter() {
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
            self.code.push((name.clone(), CodeObjectRef::from(co)));
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
        co.code = code;
        new.set(&"main".into(), co);
        new
    }

    pub fn code(&self) -> CodeObjectRef {
        self.code
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, code)| code.clone())
            .unwrap()
    }

    pub fn slots(&self) -> &Vec<(Name, CodeObjectRef)> {
        &self.code
    }

    pub fn slots_mut(&mut self) -> &mut Vec<(Name, CodeObjectRef)> {
        &mut self.code
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unit(slots: {})", self.code.len())?;
        for (name, co) in self.code.iter() {
            let co: &CodeObject = co.borrow();
            writeln!(f, "\t{}({}): {:?}", name, co.argc, co.space)?;
            for (off, inx) in co.code.iter().enumerate() {
                writeln!(f, "\t\t{}:\t {:?}", off, inx)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unit(slots: {})", self.code.len())?;
        for (name, co) in self.code.iter() {
            writeln!(f, "\t{}: {}", name, co)?;
        }
        Ok(())
    }
}
