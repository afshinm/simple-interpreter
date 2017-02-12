#[derive(Eq)]
pub enum Type {
    Integer,
    Plus,
    EOF
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Type::Integer, &Type::Integer) => true,
            (&Type::Plus, &Type::Plus) => true,
            (&Type::EOF, &Type::EOF) => true,
            _ => false
        }
    }
}

pub struct Token {
    pub _type: Type,
    pub value: String
}

impl Token {
    pub fn get_value(self) -> String {
        self.value
    }

    pub fn get_type(self) -> Type {
        self._type
    }
}
