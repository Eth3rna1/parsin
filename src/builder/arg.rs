use crate::Type;

/// Defines an Arg within [Context](struct@crate::Context)
///
/// ### **Fields**
/// ```text
///                                         Description when display
///                                         the help message
///                                                 |
///           Name of the argument                  |     Indicates whether the argument
///                      |                          |         MUST be given
///                      |                          |              |
///                     vvv                        vvv            vvv
/// let arg = Arg::new("age", Type::Int, "Your age description", true, None);
///                              ^^^                                   ^^^
///                               |                                     |
///                           Return type                               |
///                                                            Default value represented
///                                                           as a string wrapped in an Option
///                                                          enum to parse later as a substitute
///
/// ```
///
/// ### Getting Started
/// Initializing [Arg]
/// ```rust
/// use parsin::{Type, Arg};
///
/// # fn main() {
/// let arg = Arg::new(
///     String::from("name"),
///     Type::Str,
///     String::from("This is the description"),
///     false,
///     Some(String::from("default value"))
/// );
/// # }
/// ```
/// Alternatively, a simpler and more flexible initialization would be utilizing the [`From`] trait.
/// ```rust
/// use parsin::{Type, Arg};
/// # fn main() {
/// let arg = Arg::from(
///     ("name", Type::Str, "This is the description", true, Some("default value"))
/// );
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Arg {
    /// The name of the argument
    pub(crate) name: String,
    /// Defines the return type for the parsed value
    pub(crate) r#type: Type,
    /// Help message to display when errored
    pub(crate) help: String,
    /// Signal if the argument is mandatory or not
    pub(crate) is_mandatory: bool,
    /// Introduce a string default value, that'll later get parse during execution
    pub(crate) default: Option<String>,
}

impl Arg {
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_type(&self) -> Type {
        self.r#type.clone()
    }

    pub fn get_description(&self) -> String {
        self.help.clone()
    }

    pub fn is_mandatory(&self) -> bool {
        self.is_mandatory
    }

    pub fn get_default(&self) -> Option<String> {
        self.default.clone()
    }
}

impl From<&(&str, Type, &str, bool, Option<&str>)> for Arg {
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

impl From<(&str, Type, &str, bool, Option<&str>)> for Arg {
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
