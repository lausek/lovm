use super::*;

macro_rules! spawn {
    ($pool:expr, $obj:expr) => {{
        $pool.last_handle += 1;
        $pool.handles.insert($pool.last_handle, Box::new($obj));
        $pool.last_handle
    }};
}

#[derive(Clone, Debug)]
pub struct ObjectPool {
    last_handle: ObjectId,
    handles: HashMap<ObjectId, Box<dyn ObjectProtocol>>,
}

impl ObjectPool {
    pub fn new() -> Self {
        Self {
            last_handle: 0,
            handles: HashMap::new(),
        }
    }

    //pub fn new_handle(&mut self) -> ObjectId {
    //    spawn!(self, Object::new_value())
    //}

    pub fn new_handle_with_assoc(&mut self, unit: UnitRef) -> ObjectId {
        spawn!(self, Object::new_value_assoc(unit))
    }

    pub fn new_dict_handle(&mut self) -> ObjectId {
        spawn!(self, Dict::new())
    }

    pub fn new_array_handle(&mut self) -> ObjectId {
        spawn!(self, Array::new())
    }

    pub fn dispose_handle(&mut self, id: &ObjectId) {
        self.handles.remove(id);
    }

    pub fn get(&self, id: &ObjectId) -> Option<&ObjectRef> {
        self.handles.get(id)
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut ObjectRef> {
        self.handles.get_mut(id)
    }
}
