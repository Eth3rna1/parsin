use crate::Type;

/// Defines a Flag within [Context][struct@crate::Context]
#[derive(Debug, Clone)]
pub struct Flag {
    /// The flag itself used to indicate a flag
    pub name: String,
    /// The type of the value obtained to parse into
    pub r#type: Type,
    /// Help message when displaying an advanced error
    pub help: String,
    /// Signals if its an important flag or not
    pub is_mandatory: bool,
}

impl Flag {
    /// Initializes an instance of Flag
    pub fn new(name: String, r#type: Type, help: String, is_mandatory: bool) -> Self {
        Self {
            name,
            r#type,
            help,
            is_mandatory,
        }
    }
}

impl From<(String, Type, String, bool)> for Flag {
    fn from(_tuple: (String, Type, String, bool)) -> Self {
        Self::new(_tuple.0, _tuple.1, _tuple.2, _tuple.3)
    }
}

impl From<&(String, Type, String, bool)> for Flag {
    fn from(_tuple: &(String, Type, String, bool)) -> Self {
        Self::new(
            _tuple.0.clone(),
            _tuple.1.clone(),
            _tuple.2.clone(),
            _tuple.3,
        )
    }
}

impl From<&(&str, Type, &str, bool)> for Flag {
    fn from(_tuple: &(&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
        )
    }
}

impl From<(&str, Type, &str, bool)> for Flag {
    fn from(_tuple: (&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
        )
    }
}
