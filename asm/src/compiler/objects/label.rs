use super::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Label {
    pub decl: Option<(Ident, usize)>,
    pub locations: Vec<(Ident, usize)>,
    pub parent: Option<Rc<RefCell<Label>>>,
    pub public: bool,
}

impl Label {
    pub fn new() -> Self {
        Self {
            decl: None,
            locations: vec![],
            parent: None,
            public: false,
        }
    }

    pub fn declaration(mut self, ident: &Ident, off: usize) -> Self {
        self.decl = Some((ident.clone(), off));
        self
    }

    pub fn location(mut self, ident: &Ident, off: usize) -> Self {
        self.locations.push((ident.clone(), off));
        self
    }

    pub fn is_exported(&self) -> bool {
        (self.parent.is_none() && self.public)
            || self
                .decl
                .as_ref()
                .and_then(|(ident, _)| Some(ident.raw == "main"))
                .unwrap_or(false)
    }
}
