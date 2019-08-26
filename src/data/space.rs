use super::*;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Space {
    pub consts: Vec<Value>,
    pub locals: Vec<Name>,
    pub globals: Vec<Name>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
        }
    }

    pub fn merge(&mut self, other: &Self) {
        for oconst in other.consts.iter() {
            gen::index_of(&mut self.consts, oconst);
        }
        for olocal in other.locals.iter() {
            gen::index_of(&mut self.locals, olocal);
        }
        for oglobal in other.globals.iter() {
            gen::index_of(&mut self.globals, oglobal);
        }
    }
}

impl std::fmt::Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut children = vec![];
        if !self.consts.is_empty() {
            children.push(format!("consts: {:?}", self.consts));
        }
        if !self.locals.is_empty() {
            children.push(format!("locals: {:?}", self.locals));
        }
        if !self.globals.is_empty() {
            children.push(format!("globals: {:?}", self.globals));
        }
        write!(f, "Space({})", children.join(", "))
    }
}
