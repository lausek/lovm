use super::*;

use serde::de::*;
use serde::*;

// if we return a `Unit` in lookup calls, we give the promise that it stays
// valid for the `run_object` call aswell. however, the vm could decide to change
// the object inside its `Unit` thus violating the given lifetime promise. we
// therefore wrap everything inside a reference counter.

#[derive(Clone, Debug, PartialEq)]
pub struct UnitRef(Rc<Unit>);

impl UnitRef {
    pub fn get_mut(&mut self) -> &mut Unit {
        Rc::make_mut(&mut self.0)
    }
}

impl From<Unit> for UnitRef {
    fn from(from: Unit) -> Self {
        Self(Rc::new(from))
    }
}

impl Borrow<Unit> for UnitRef {
    fn borrow(&self) -> &Unit {
        &self.0
    }
}

impl std::fmt::Display for UnitRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
