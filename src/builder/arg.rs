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
    /// Introduce a string default value, that'll later get parse during execution
    pub default: Option<String>,
}

impl Arg {
    pub fn new(
        name: &str,
        r#type: Type,
        help: &str,
        is_mandatory: bool,
        default: Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            r#type,
            help: help.to_string(),
            is_mandatory,
            default,
        }
    }
}

impl From<(String, Type, String, bool, Option<String>)> for Arg {
    fn from(_t: (String, Type, String, bool, Option<String>)) -> Self {
        Self::new(&_t.0, _t.1, &_t.2, _t.3, _t.4)
    }
}

impl From<&(String, Type, String, bool, Option<String>)> for Arg {
    fn from(_t: &(String, Type, String, bool, Option<String>)) -> Self {
        Self::new(&_t.0, _t.1.clone(), &_t.2, _t.3, _t.4.clone())
    }
}

impl From<&(&str, Type, &str, bool, Option<String>)> for Arg {
    fn from(_tuple: &(&str, Type, &str, bool, Option<String>)) -> Self {
        Self::new(
            _tuple.0,
            _tuple.1.clone(),
            _tuple.2,
            _tuple.3,
            _tuple.4.clone(),
        )
    }
}

impl From<(&str, Type, &str, bool, Option<String>)> for Arg {
    fn from(_tuple: (&str, Type, &str, bool, Option<String>)) -> Self {
        Self::new(_tuple.0, _tuple.1, _tuple.2, _tuple.3, _tuple.4)
    }
}

impl From<&(&str, Type, &str, bool, Option<&str>)> for Arg {
    fn from(_tuple: &(&str, Type, &str, bool, Option<&str>)) -> Self {
        if let Some(string) = _tuple.4 {
            return Self::new(
                _tuple.0,
                _tuple.1.clone(),
                _tuple.2,
                _tuple.3,
                Some(string.to_string()),
            );
        }
        Self::new(_tuple.0, _tuple.1.clone(), _tuple.2, _tuple.3, None)
    }
}

impl From<(&str, Type, &str, bool, Option<&str>)> for Arg {
    fn from(_tuple: (&str, Type, &str, bool, Option<&str>)) -> Self {
        if let Some(string) = _tuple.4 {
            return Self::new(
                _tuple.0,
                _tuple.1,
                _tuple.2,
                _tuple.3,
                Some(string.to_string()),
            );
        }
        Self::new(_tuple.0, _tuple.1, _tuple.2, _tuple.3, None)
    }
}
