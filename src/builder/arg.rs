use crate::Type;


/// Defines an Arg within [Context](struct@crate::Context)
#[derive(Debug, Clone)]
pub struct Arg {
    /// The name of the argument
    pub name: String,
    /// The type to parse the argument given into
    pub r#type: Type,
    /// Help message to display when errored
    pub help: String,
    /// Signal if the argument is mandatory or not
    pub is_mandatory: bool,
}

impl Arg {
    pub fn new(name: &str, r#type: Type, help: &str, is_mandatory: bool) -> Self {
        Self {
            name : name.to_string(),
            r#type,
            help : help.to_string(),
            is_mandatory,
        }
    }
}

impl From<(String, Type, String, bool)> for Arg {
    fn from(_t: (String, Type, String, bool)) -> Self {
        Self::new(&_t.0, _t.1, &_t.2, _t.3)
    }
}

impl From<&(String, Type, String, bool)> for Arg {
    fn from(_t: &(String, Type, String, bool)) -> Self {
        Self::new(&_t.0, _t.1.clone(), &_t.2, _t.3)
    }
}

impl From<&(&str, Type, &str, bool)> for Arg {
    fn from(_tuple: &(&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0,
            _tuple.1.clone(),
            _tuple.2,
            _tuple.3,
        )
    }
}

impl From<(&str, Type, &str, bool)> for Arg {
    fn from(_tuple: (&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0,
            _tuple.1,
            _tuple.2,
            _tuple.3,
        )
    }
}
