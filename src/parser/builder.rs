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

    pub fn unwrap_str<'a>(&'a self) -> &'a String {
        match self {
            Value::Str(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }

    pub fn unwrap_str_or<'a>(&'a self, other : &'a String) -> &'a String {
        if self.is_null() {
            return other;
        }
        self.unwrap_str()
    }

    pub fn unwrap_bool<'a>(&'a self) -> &'a bool {
        match self {
            Value::Bool(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }

    pub fn unwrap_bool_or<'a>(&'a self, other : &'a bool) -> &'a bool {
        if self.is_null() {
            return other;
        }
        self.unwrap_bool()
    }

    pub fn unwrap_int(&self) -> &i32 {
        match self {
            Value::Int(v) => v,
            _ => panic!("Unwrapped the wrong variant"),
        }
    }

    pub fn unwrap_int_or<'a>(&'a self, other : &'a i32) -> &'a i32 {
        if self.is_null() {
            return other;
        }
        self.unwrap_int()
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
