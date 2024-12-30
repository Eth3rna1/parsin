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
    /// Introduce a string default value, that'll get parsed later during execution
    pub default: Option<String>,
}

impl Flag {
    /// Initializes an instance of Flag
    pub fn new(
        name: String,
        r#type: Type,
        help: String,
        is_mandatory: bool,
        default: Option<String>,
    ) -> Self {
        Self {
            name,
            r#type,
            help,
            is_mandatory,
            default,
        }
    }
}

impl From<(String, Type, String, bool, Option<String>)> for Flag {
    fn from(_tuple: (String, Type, String, bool, Option<String>)) -> Self {
        Self::new(_tuple.0, _tuple.1, _tuple.2, _tuple.3, _tuple.4)
    }
}

impl From<&(String, Type, String, bool, Option<String>)> for Flag {
    fn from(_tuple: &(String, Type, String, bool, Option<String>)) -> Self {
        Self::new(
            _tuple.0.clone(),
            _tuple.1.clone(),
            _tuple.2.clone(),
            _tuple.3,
            _tuple.4.clone(),
        )
    }
}

impl From<(&str, Type, &str, bool, Option<String>)> for Flag {
    fn from(_tuple: (&str, Type, &str, bool, Option<String>)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
            _tuple.4,
        )
    }
}

impl From<&(&str, Type, &str, bool, Option<String>)> for Flag {
    fn from(_tuple: &(&str, Type, &str, bool, Option<String>)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
            _tuple.4.clone(),
        )
    }
}

impl From<&(&str, Type, &str, bool, Option<&str>)> for Flag {
    fn from(_tuple: &(&str, Type, &str, bool, Option<&str>)) -> Self {
        if let Some(string) = _tuple.4 {
            return Self::new(
                _tuple.0.to_string(),
                _tuple.1.clone(),
                _tuple.2.to_string(),
                _tuple.3,
                Some(string.to_string()),
            );
        }
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
            None,
        )
    }
}

impl From<(&str, Type, &str, bool, Option<&str>)> for Flag {
    fn from(_tuple: (&str, Type, &str, bool, Option<&str>)) -> Self {
        if let Some(string) = _tuple.4 {
            return Self::new(
                _tuple.0.to_string(),
                _tuple.1,
                _tuple.2.to_string(),
                _tuple.3,
                Some(string.to_string()),
            );
        }
        Self::new(
            _tuple.0.to_string(),
            _tuple.1,
            _tuple.2.to_string(),
            _tuple.3,
            None,
        )
    }
}
