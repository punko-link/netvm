use crate::{Graph, List, Var};

impl Graph {
    pub fn new() -> Graph {
        Graph (Vec::new())
    }
}



impl Graph {
    pub fn filter_one(self, filter: List) -> Var {

        let keys = filter.clone().get_keys();

        for (addr, one) in self.0.iter().enumerate() {
                let l = one.clone();
                let mut is = true;

                for k in &keys {
                    let v = l.0.get(&k.clone());

                    if v.is_none() || v.unwrap() != &filter.clone().get(k.clone()) {
                        is = false;
                    }
                }

                if is == true { return  Var::Pointer(addr) }

        }

        Var::Undefined
    }
}

impl Graph {
    pub fn create_node(mut self, node: List) -> (Graph, Var) {
        self.0.push(node);
        (self.clone(), Var::Pointer(self.0.len()))
    }
    pub fn get_node(self, addr: Var) -> List {
        self.0[addr.value_of_pointer()].clone()
    }

    pub fn mod_node(mut self, addr: Var, node: List) {
        self.0[addr.value_of_pointer()] = node;
    }

    pub fn rm_node(mut self, addr: Var) -> List {
        self.0.remove(addr.value_of_pointer())
    }
}
