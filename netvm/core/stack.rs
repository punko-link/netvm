use core::{ Stack, Var };



impl Stack {
    pub fn new(start_stack: Vec<Var>) -> Stack {
        Stack(start_stack)
    }
}
impl Stack {
    pub fn take(&mut self) -> Var {
        self.0.pop().unwrap()
    }

    pub fn push(&mut self, var: Var) {
        self.0.push(var);
    }
}

