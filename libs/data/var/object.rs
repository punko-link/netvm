use std::collections::HashMap;
use crate::var::{Object, Var, VarType};

impl Object {
    pub fn set(mut self, key: String, value: Var) -> Object {
        self.data.insert(key, value);

        self
    }
}

impl Var {
    pub fn object() -> Var {
        Var { single_data: Box::new(None), vector_data: Box::new(Some(Object { data: HashMap::new() })), var_type: VarType::Object, }
    }
}

impl Var {
    pub fn object_set(mut self, key: String, value: Var) -> Var {
        let old = self.vector_data.unwrap();
        let new = old.set(key, value);

        self.vector_data = Box::from(Some(new));

        self
    }

    pub fn object_get(self, key: String) -> Var {
        let var = self.vector_data.unwrap().data.get(&key);
        if var.is_none() { return self.return_undefined() }

        var.unwrap().clone()
    }

    pub fn object_remove(mut self, key: String) {
        self.vector_data.unwrap().data.remove(&key);
    }
}

impl Var {
    // pub fn object_dive_get(self) -> Var {
    //     let current = self.clone();
    //     loop {
    //
    //     }
    // }
}