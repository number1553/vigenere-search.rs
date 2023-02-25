#[derive(Debug, Eq, Clone, Hash)]
pub struct Token {
    pub value: String,
}

impl Token {
    pub fn new(value: String) -> Self {
        Token {
            value
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.value == other.value
    }
}