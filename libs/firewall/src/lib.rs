use std::collections::HashMap;

mod handle_connection;
mod router;

#[derive(Clone, PartialEq)]

enum Access {
    Public,
    Private(String)
}

#[derive(Clone)]
pub struct RouteParams {
    access: Access,
    rsa: bool,
    entry: String
}

#[derive(Clone)]
pub struct Firewall {
    routes: HashMap<String, RouteParams>
}

impl Firewall {
    pub fn new() -> Firewall {
        Firewall { routes: HashMap::new() }
    }
}
