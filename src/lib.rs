/*! # Parsin
Parsin is a minimalistic Command Line Interface parser for Rust.
Parsin offers a simple way to creating context to parse that is both
fast and simple.

## Getting Started
Add parsin into your cargo project by using this command
```console
cargo add parsin
```
Once the crate has been added include these imports
```rust
use parsin::{
    Context,
    Type,
    parse
};
```
Initialize [`Context`]
```rust
# use parsin::{
#    Context,
#    Type,
#    parse
# };
#
# fn main() {
let context = Context::from((
    &[
        ("Argument", Type::Str, "test argument desc", true, Some("string default value"))
    ],
    &[
        ("--flag", Type::Bool, "test flag desc", false, None)
    ]
));
# }
```
Pass a reference of [`Context`] into [`parse`].
```rust
# use parsin::{
#    Context,
#    Type,
#    parse
# };
use parsin::parser::ParsedArguments;

#
# fn main() {
# let context = Context::new();
// Final object
// contains `.flags` and `.arguments` which contain
// hashmaps whose values are Value (enum@crate::parser::Value)
let parsed: ParsedArguments = parse(&context);
# }
```
# Working with Parsin
Regarding cargo projects, if you wish to pass arguments during the
building process, when you run `cargo run --release` or `cargo run`,
you can insert `--` between the cargo command and your arguments.
Take the following for example:
```console
cargo run --release -- {ARGUMENT} {FLAG1} {FLAG2}
```
Or
```console
cargo run -- {ARGUMENT} {FLAG1} {FLAG2}
```

## Examples
Initiating [`Context`]
```rust
use parsin::{Context, Type};

# fn main() {
let ctx = Context::from(( // within a tuple
    &[ // First list defines the arguments
        ("name", Type::Str, "Your name", true, None), // Arg
    ],
    &[ // Second list defines the flags
        ("--repeat", Type::Int, "The amount of times to greet", false, Some("1")), // Flag
    ]
));
# }
```
Alternatively, a more manual approach would include the usage of [`Arg`] and [`Flag`].
```rust
use parsin::{Context, Type, Flag, Arg};

# fn main() {
let ctx = Context::from((
    &[
        Arg::from(("arg1", Type::Str, "argument one", true, None)),
        Arg::from(("arg2", Type::Str, "argument two", true, None)),
    ],
    &[
        Flag::from(("flag1", Type::Int, "flag with int value", false, Some("19"))),
        Flag::from(("flag2", Type::Bool, "once raised, returns true for this flag", false, Some("false"))),
    ]
));
# }
```

*/
// Made modules in this crate
mod builder;
/// Contains its own Error struct along with error types.
pub mod errors;
mod help;
/// Contains the necessary objects to work with [`ParsedArgument`](struct@crate::parser::ParsedArguments)
pub mod parser;

// Simplifying modularization within the API
pub use builder::arg::Arg;
pub use builder::context::Context;
pub use builder::flag::Flag;
pub use parser::parse;

// Use of the standard library
use std::env::args;

/// Obtains the Command Line arguments
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
