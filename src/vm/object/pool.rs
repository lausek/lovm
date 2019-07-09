use super::*;

#[derive(Clone, Debug)]
pub struct ObjectPool {
    last_handle: ObjectId,
    handles: HashMap<ObjectId, Object>,
}

impl ObjectPool {
    pub fn new() -> Self {
        Self {
            last_handle: 0,
            handles: HashMap::new(),
        }
    }

    pub fn new_handle(&mut self) -> ObjectId {
        self.last_handle += 1;
        self.handles.insert(self.last_handle, Object::new_value());
        self.last_handle
    }

    pub fn new_handle_with_assoc(&mut self, unit: UnitRef) -> ObjectId {
        self.last_handle += 1;
        self.handles
            .insert(self.last_handle, Object::new_value_assoc(unit));
        self.last_handle
    }

    pub fn new_dict_handle(&mut self) -> ObjectId {
        self.last_handle += 1;
        self.handles.insert(self.last_handle, Object::new_dict());
        self.last_handle
    }

    pub fn new_array_handle(&mut self) -> ObjectId {
        self.last_handle += 1;
        self.handles.insert(self.last_handle, Object::new_array());
        self.last_handle
    }

    pub fn dispose_handle(&mut self, id: &ObjectId) {
        self.handles.remove(id);
    }

    pub fn get(&self, id: &ObjectId) -> Option<&Object> {
        self.handles.get(id)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut Object> {
        self.handles.get_mut(id)
    }
}
