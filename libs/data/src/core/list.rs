use std::collections::HashMap;
use crate::{List, Var};

impl List {
    pub fn new() -> List {
        List (HashMap::new())
    }

    pub fn set(mut self, k: String, v: Var) {
        self.0.insert(k, v);
    }

    pub fn get(self, k: String) -> Var {
        let v = self.0.get(&k);
        if v.is_some() { return v.unwrap().clone() };

        Var::Undefined
    }

    pub fn rm(mut self, k: String) -> Var {
        let v = self.0.remove(&k);
        if v.is_some() { return v.unwrap().clone() };

        Var::Undefined
    }

    pub fn get_keys(self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();

        for key in self.0.keys() {
            keys.push(key.clone())
        }

        keys
    }
}
