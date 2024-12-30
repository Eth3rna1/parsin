use std::collections::HashMap;

/// Encapsulates the returned parsed argument
#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int(i32),
    Null,
}

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn unwrap_str(&self) -> &String {
        match self {
            Value::Str(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }

    pub fn unwrap_bool(&self) -> &bool {
        match self {
            Value::Bool(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }

    pub fn unwrap_int(&self) -> &i32 {
        match self {
            Value::Int(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }
}

/// The returned parsed data
#[derive(Debug, Clone)]
pub struct ParsedArguments {
    pub arguments: HashMap<String, Value>,
    pub flags: HashMap<String, Value>,
}

impl ParsedArguments {
    pub fn new() -> Self {
        Self {
            arguments: HashMap::new(),
            flags: HashMap::new(),
        }
    }
}

impl Default for ParsedArguments {
    fn default() -> Self {
        Self::new()
    }
}
