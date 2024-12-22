/*
    TO-DO: move all the implementations into a module
*/
pub mod errors;
mod extra;
mod help;

use errors::Error;
use errors::ErrorKind;
use help::send_advanced_help_and_exit;
use help::send_help_and_exit;

use std::collections::HashMap;
use std::convert::From;
use std::convert::Into;
use std::env::args;
use std::slice::Iter;

pub fn get_env_args() -> Vec<String> {
    args().collect::<Vec<String>>()
}

/// An enum used to signal what
/// to parse a value into
#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Str,
    Bool,
}

/// Meant to be used within the
/// Context struct before parsing arguments
#[derive(Debug, Clone)]
pub struct Flag {
    pub name: String,
    pub r#type: Type,
    pub help: String,
    mandatory: bool,
}

impl Flag {
    pub fn new(name: String, r#type: Type, help: String, mandatory: bool) -> Self {
        Self {
            name,
            r#type,
            help,
            mandatory,
        }
    }

    pub fn is_mandatory(&self) -> bool {
        self.mandatory
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

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub r#type: Type,
    pub help: String,
    mandatory: bool,
}

impl Arg {
    pub fn new(name: String, r#type: Type, help: String, mandatory: bool) -> Self {
        Self {
            name,
            r#type,
            help,
            mandatory,
        }
    }

    pub fn is_mandatory(&self) -> bool {
        self.mandatory
    }
}

impl From<(String, Type, String, bool)> for Arg {
    fn from(_t: (String, Type, String, bool)) -> Self {
        Self::new(_t.0, _t.1, _t.2, _t.3)
    }
}

impl From<&(String, Type, String, bool)> for Arg {
    fn from(_t: &(String, Type, String, bool)) -> Self {
        Self::new(_t.0.clone(), _t.1.clone(), _t.2.clone(), _t.3)
    }
}

impl From<&(&str, Type, &str, bool)> for Arg {
    fn from(_tuple: &(&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
        )
    }
}

impl From<(&str, Type, &str, bool)> for Arg {
    fn from(_tuple: (&str, Type, &str, bool)) -> Self {
        Self::new(
            _tuple.0.to_string(),
            _tuple.1.clone(),
            _tuple.2.to_string(),
            _tuple.3,
        )
    }
}

/// Context
#[derive(Debug, Clone)]
pub struct Context {
    name: String,
    description: String,
    args: Vec<Arg>,
    flags: Vec<Flag>,
}

impl Context {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            args: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn add_args(&mut self, args: &[Arg]) -> Result<(), Error> {
        for arg in args {
            self.add_arg(arg.clone())?;
        }
        Ok(())
    }

    pub fn add_flags(&mut self, flags: &[Flag]) -> Result<(), Error> {
        for flag in flags {
            self.add_flag(flag.clone())?;
        }
        Ok(())
    }

    pub fn contains_arg(&self, name: &str) -> bool {
        for arg in self.args.iter() {
            if arg.name == name {
                return true;
            }
        }
        false
    }

    pub fn get_arg(&self, name: &str) -> Option<Arg> {
        for arg in self.args.iter() {
            if arg.name == name {
                return Some(arg.clone());
            }
        }
        None
    }

    pub fn add_arg(&mut self, arg: Arg) -> Result<(), Error> {
        if self.contains_arg(&arg.name) {
            return Err(Error::new(
                ErrorKind::DuplicateArgument,
                format!("Found a duplicate argument for `{}`", &arg.name),
            ));
        }
        self.args.push(arg);
        Ok(())
    }

    pub fn remove_arg(&mut self, name: &str) -> Result<Arg, Error> {
        if !self.contains_arg(name) {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                format!("Argument `{}` never existed within the context", name),
            ));
        }
        let index: usize = {
            let mut bind = 0;
            for (i, ar) in self.args.iter().enumerate() {
                if ar.name == name {
                    bind = i;
                }
            }
            bind
        };
        Ok(self.args.remove(index))
    }

    pub fn contains_flag(&self, name: &str) -> bool {
        for flag in self.flags.iter() {
            if flag.name == name {
                return true;
            }
        }
        false
    }

    pub fn get_flag(&self, name: &str) -> Option<Flag> {
        for flag in self.flags.iter() {
            if flag.name == name {
                return Some(flag.clone());
            }
        }
        None
    }

    pub fn add_flag(&mut self, flag: Flag) -> Result<(), Error> {
        if self.contains_flag(&flag.name) {
            return Err(Error::new(
                ErrorKind::DuplicateFlag,
                format!("Found a duplicate flag for `{}`", &flag.name),
            ));
        }
        self.flags.push(flag);
        Ok(())
    }

    pub fn remove_flag(&mut self, name: &str) -> Result<Flag, Error> {
        if !self.contains_flag(name) {
            return Err(Error::new(
                ErrorKind::MissingFlag,
                format!("Flag `{}` never existed within the context", name),
            ));
        }
        let index: usize = {
            let mut bind = 0;
            for i in 0..self.flags.len() {
                if self.flags[i].name == name {
                    bind = i;
                    break;
                }
            }
            bind
        };
        Ok(self.flags.remove(index))
    }
}

/// A struct containing the instructions to parse
/// and retrieve such data. It contains different flags
/// that might contain string, integer, or boolean values,
/// along the arguments hashmap.
#[derive(Debug, Clone)]
pub struct ParsedArguments {
    /// lone arguments that weren't part of a value for a flag, will go here
    pub arguments: HashMap<String, Option<String>>,
    pub flags_int: HashMap<String, i32>,
    pub flags_str: HashMap<String, String>,
    pub flags_bool: HashMap<String, bool>,
}

impl ParsedArguments {
    pub fn new() -> Self {
        Self {
            arguments: HashMap::new(),
            flags_int: HashMap::new(),
            flags_str: HashMap::new(),
            flags_bool: HashMap::new(),
        }
    }
}

impl Default for ParsedArguments {
    fn default() -> Self {
        Self::new()
    }
}

/// parsing function that does the actual parsing
///
/// Step By Step Process
/// --------------------
///     -> Get arguments and check for necessary arguments
///     -> Assign arguments
///     -> Separate flags and values from boolean flags
///     -> Parse boolean flags
///     -> Parse flags and values
fn _parse(arguments: &[String], __context: Context) -> Result<ParsedArguments, Error> {
    if arguments.contains(&String::from("--help")) {
        return Err(Error::new(ErrorKind::WantsHelp, String::new()));
    }
    // lone arguments get stored separately
    let mut __args: Vec<&str> = Vec::new();
    // boolean flags get stored separately just in case,
    // a flag is passed as a value
    let mut __boolean_flags: Vec<Flag> = Vec::new();
    let mut __flags_and_values: Vec<String> = arguments.to_vec();
    {
        // checking for ungiven mandatory arguments

        // All variables initialized within these curly
        // braces are explicit to this scope
        let mut __pointer: usize = 0;
        // memoizing arguments length
        let a_len = arguments.len();
        while __pointer < a_len {
            if let Some(flag) = __context.get_flag(&arguments[__pointer]) {
                // taking advantage of iterations to separate flags and values

                // With flags that intake some sort of value,
                // I don't want to deal with the issue of the client providing
                // a value for a flag or not, so I'm going to do is
                // delete elements from the cloned arguments variable (__flags_and_values)
                // and at the end, I'll end up with the arguments to parse
                match flag.r#type {
                    Type::Bool => {
                        {
                            // finding and removing from flags and values variable
                            extra::find_and_remove(&mut __flags_and_values, &flag.name)?;
                        }
                        __pointer += 1;
                        __boolean_flags.push(flag);
                    }
                    // no need to remove anything since I want the flags and values to stay in the vector
                    Type::Int | Type::Str => {
                        // if pointer stays within boundries and the next iteration is "--"
                        if __pointer + 2 <= a_len && &arguments[__pointer + 1] == "--" {
                            __pointer += 3;
                            continue;
                        }
                        __pointer += 2;
                    }
                }
                continue;
            }
            // ELSE: not a flag or value
            //extra::find_and_remove(&mut __flags_and_values, &arguments[__pointer])?;
            __args.push(&arguments[__pointer]);
            __pointer += 1;
        }
        // if the arguments length is equal to or greater
        // than the specified arguments in the context,
        // than iterating is redundant and should continue
        // on to the next step.
        // This if statement is check for any lacking arguments
        // with the arguments given
        /*
            ARGUMENTS IN CONTEXT: [a, b, c, d]
                 GIVEN ARGUMENTS: [a, b, c, _]
                                           ^^^
                                            |
                                       Missing Argument
        */
        // memoizing lengths
        let a_len = __args.len();
        let c_len = __context.args.len();
        if a_len < c_len {
            for i in __pointer..c_len {
                let arg = &__context.args[i];
                // argument is mandatory and its already assumed that the argument was not provided
                if arg.is_mandatory() {
                    return Err(Error::new(
                        ErrorKind::MissingArgument,
                        format!("Missing argument `<{}>`", arg.name.to_uppercase()),
                    ));
                }
            }
        }
    }
    {
        // checking for mandatory ungiven flags
        for flag in __context.flags.iter() {
            if flag.is_mandatory() && !arguments.contains(&flag.name) {
                return Err(Error::new(
                    ErrorKind::MissingFlag,
                    format!("Missing flag `{}`", &flag.name),
                ));
            }
        }
    }
    let mut __parsed = ParsedArguments::new();
    {
        // moving boolean flags that were cached previously
        for flag in __boolean_flags.iter() {
            __parsed.flags_bool.insert(flag.name.clone(), true);
        }
    }
    {
        // parsing flags and values
        // using a pointer approach
        let mut __pointer: usize = 0;
        // memoizing the length
        let fv_len = __flags_and_values.len();
        while __pointer < fv_len {
            let element = &__flags_and_values[__pointer];
            if let Some(flag) = __context.get_flag(element) {
                match flag.r#type {
                    Type::Int => {
                        if __pointer + 1 > fv_len {
                            return Err(Error::new(
                                ErrorKind::MissingValue,
                                format!("Missing integer value for `{}`.", &flag.name),
                            ));
                        }
                        let int_result = __flags_and_values[__pointer + 1].parse::<i32>();
                        if let Err(error) = int_result {
                            return Err(Error::new(ErrorKind::Other, error.to_string()));
                        }
                        __parsed.flags_int.insert(flag.name, int_result.unwrap());
                        __pointer += 2;
                    }
                    Type::Str => {
                        if __pointer + 1 > fv_len {
                            return Err(Error::new(
                                ErrorKind::MissingValue,
                                format!("Missing value for `{}`.", &flag.name),
                            ));
                        }
                        // Memoized variable; upcoming value
                        let upc_val = &__flags_and_values[__pointer + 1];
                        // checking if a flag was passed in as a value
                        if __context.contains_flag(upc_val) {
                            return Err(Error::new(ErrorKind::Other, format!("Missing value for `{}`, if you wish to pass `{}` as the value, try passing `-- {}`.", &flag.name, upc_val, upc_val)));
                        }
                        if upc_val == "--" {
                            if __pointer + 2 > fv_len {
                                return Err(Error::new(
                                    ErrorKind::MissingValue,
                                    format!("Missing value for `{}`.", &flag.name),
                                ));
                            }
                            __parsed
                                .flags_str
                                .insert(flag.name, __flags_and_values[__pointer + 2].to_owned());
                            __pointer += 3;
                            continue;
                        }
                        __parsed.flags_str.insert(flag.name, upc_val.to_string());
                        __pointer += 2;
                    }
                    /* Boolean type has already been dealt with */
                    // flag containing a boolean type shouldn't be here
                    Type::Bool => panic!("Boolean type spilled into __flags_and_values"),
                }
                // end of the match statement
                continue
            }
            // ELSE not a flag
            //__args.push(element);
            __pointer += 1;
        }
    }
    {
        // Pairing each argument with the context given

        // new memoization of __args
        let a_len = __args.len();
        for (i, arg) in __context.args.iter().enumerate() {
            if i >= a_len {
                __parsed.arguments.insert(arg.name.clone(), None);
                continue
            }
            __parsed.arguments.insert(arg.name.clone(), Some(__args[i].to_string()));
        }
    }
    dbg!(__args);
    Ok(__parsed)
}

/// Main function to be executed when parsing the CLI arguments
///
/// PARAMETERS (TYPE, NAME)
/// ----------
///     Context context => Struct containing the necessary data to parsed and retrieve, if any
pub fn parse(context: Context) -> ParsedArguments {
    // ignoring the first argument, which is automatically
    // the file being executed
    let result: Result<ParsedArguments, Error> = _parse(&get_env_args()[1..], context.clone());
    if let Err(error) = result {
        /*
            All errors have been implemented
            inside the function, no need to
            implement the error for if the
            --help flag is raised

            The function exits on its own
        */
        send_help_and_exit(&context, &error);
        // exit(1)  <<< Redundant code
    }
    result.unwrap()
}
