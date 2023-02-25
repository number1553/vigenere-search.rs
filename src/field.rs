use crate::token::Token;

#[derive(Debug, Eq, Clone)]
pub struct FieldValue {
    pub value_int: Option<i32>,
    pub value_bool: Option<bool>,
    pub value_string: Option<String>,
    pub value_tokens: Option<Vec<Token>>,
}

impl FieldValue {
    pub fn new() -> FieldValue {
        FieldValue {
            value_int: None,
            value_bool: None,
            value_string: None,
            value_tokens: None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap());
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap() as i32);
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().parse::<i32>().unwrap());
        }

        None
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap() != 0);
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap());
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().parse::<bool>().unwrap());
        }

        None
    }

    pub fn as_string(&self) -> Option<String> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap().to_string());
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap().to_string());
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().clone());
        }

        None
    }
}

impl PartialEq for FieldValue {
    fn eq(&self, other: &FieldValue) -> bool {
        self.as_string() == other.as_string()
    }
}