use crate::Var;

impl Var {
    pub fn type_name(self) -> String {
        String::from(match self {
            Self::Pointer(..) => "pointer",  // add
            Self::Number(..) => "number",    // add
            Self::Uint(..) => "uint",        // add
            Self::String(..) => "string",
            Self::Byte(..) => "byte",        // add
            Self::Bool(..) => "bool",
            Self::Cmd(..) => "cmd",
            Self::Undefined => "undefined"
        })
    }

    pub fn check_type_match(self, y: Var) -> String {
        let data_type = self.type_name();
        if data_type != y.type_name() {
            panic!("Type mismatch");
        }

        data_type
    }
}
