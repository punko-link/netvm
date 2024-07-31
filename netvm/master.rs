use stdlib::interface;

pub struct Master {
    counter: usize
}


impl Master {
    pub fn new() -> Master {
        Master { counter: 0 }
    }

    pub fn run(mut self, incoming_package: interface::Incoming) -> (interface::Outgoing, Master) {
        self.counter += 1;
        (interface::Outgoing::default(), self)
    }

    pub fn print_counter(self) -> Master {
        println!("{}", self.counter);
        self
    }
}
