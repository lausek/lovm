use super::*;

use serde::de::*;
use serde::*;

// if we return a `CodeObject` in lookup calls, we give the promise that it stays
// valid for the `run_object` call aswell. however, the vm could decide to change
// the object inside its `Unit` thus violating the given lifetime promise. we
// therefore wrap everything inside a reference counter.

#[derive(Clone, Debug, PartialEq)]
pub struct CodeObjectRef(Rc<CodeObject>);

impl CodeObjectRef {
    pub fn get_mut(&mut self) -> &mut CodeObject {
        Rc::make_mut(&mut self.0)
    }
}

impl From<CodeObject> for CodeObjectRef {
    fn from(from: CodeObject) -> Self {
        Self(Rc::new(from))
    }
}

impl Borrow<CodeObject> for CodeObjectRef {
    fn borrow(&self) -> &CodeObject {
        &self.0
    }
}

impl std::fmt::Display for CodeObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Serialize for CodeObjectRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let co: &CodeObject = self.borrow();
        co.serialize(serializer)
    }
}

struct CodeObjectRefVisitor;

impl<'de> Visitor<'de> for CodeObjectRefVisitor {
    type Value = CodeObject;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a code object ref")
    }
}

impl<'de> Deserialize<'de> for CodeObjectRef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let co = deserializer.deserialize_any(CodeObjectRefVisitor)?;
        Ok(CodeObjectRef::from(co))
    }
}
