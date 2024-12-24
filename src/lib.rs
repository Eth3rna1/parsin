/*! # Parsin
Parsin is a minimalistic Command Line Interface for Rust.
Parsin offers explicit declaration of flags and arguments for
better readability and maintainability within client code.
The only functions that the client would need is [struct@Context],
[struct@Flag], [struct@Arg], [enum@Type], and [fn@parse].
## Examples

Initializing [`Arg`](struct@crate::Arg)
```rust
use parsin::{Type, Arg};

fn main() {
    let arg = Arg::new("Name", "This is the description", Type::Str, true);
    println!("{:#?}", arg);
}
```
Outputs
```text
Arg {
    name: "Name",
    type: Str,
    help: "This is the description",
    is_mandatory: true,
}
```

Initializing [`Context`]
```rust
fn main() {
    let context = Context::new()
}

```
*/
// Made modules in this program
/// Contains its own Error struct along with error types
pub mod errors;
mod extra;
mod help;
mod builder;

// Simplifying the use of functions
// within the created modules
use errors::Error;
use errors::ErrorKind;
use help::send_help_and_exit;

// Simplifying modularization within the API
pub use builder::context::Context;
pub use builder::flag::Flag;
pub use builder::arg::Arg;

// Use of the standard library
use std::collections::HashMap;
use std::convert::From;
use std::env::args;

/// Obtains the Command Line Interface arguments
pub fn get_env_args() -> Vec<String> {
    args().collect::<Vec<String>>()
}

/// Defines what to **parse** an argument into
#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Str,
    Bool,
}

/// Encapsulates the returned parsed argument
#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int(i32)
}

/// The returned parsed data
#[derive(Debug, Clone)]
pub struct ParsedArguments {
    pub arguments: HashMap<String, Option<Value>>,
    pub flags_int: HashMap<String, i32>,
    pub flags_str: HashMap<String, String>,
    pub flags_bool: HashMap<String, bool>,
}

impl ParsedArguments {
    fn new() -> Self {
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
/// # Step By Step Process
///     - Get arguments and check for necessary arguments
///     - Assign arguments
///     - Separate flags and values from boolean flags
///     - Parse boolean flags
///     - Parse flags and values
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
                if arg.is_mandatory {
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
            if flag.is_mandatory && !arguments.contains(&flag.name) {
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
                            return Err(Error::new(ErrorKind::Other, format!("Ref: `{}`, {}", flag.name, error.to_string())));
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

        // new memoization of __args length
        let a_len = __args.len();
        for (i, arg) in __context.args.iter().enumerate() {
            if i >= a_len {
                __parsed.arguments.insert(arg.name.clone(), None);
                continue
            }
            {
                // parsing the arguments given into their respective types
                match arg.r#type {
                    Type::Int  => {
                        let result_int = __args[i].parse::<i32>();
                        if let Err(error) = result_int {
                            return Err(Error::new(ErrorKind::Other, format!("Ref: `{}`, {}", arg.name, error.to_string())));
                        }
                        __parsed.arguments.insert(arg.name.clone(), Some(Value::Int(result_int.unwrap())));
                    },
                    Type::Str  => {
                        __parsed.arguments.insert(arg.name.clone(), Some(Value::Str(__args[i].to_string())));
                    },
                    Type::Bool => {
                        __parsed.arguments.insert(arg.name.clone(), Some(Value::Bool(true)));
                    }
                }
            }
        }
    }
    Ok(__parsed)
}

/// Main function to be executed when parsing the CLI arguments
///
/// # Paramters
/// | Type    | Name    | Description |
/// |---------|---------|-------------|
/// | context | Context | Gives the parser context to what to parse |
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
