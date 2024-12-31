/*! # Parsin
Parsin is a minimalistic Command Line Interface parser for Rust.
Parsin offers a simple way to creating context to parse that is both
fast and simple.

## Getting Started
All you really need is the struct [`Context`](struct@crate::Context), along with
[`Type`] enum to define the return types for flags and arguments,
to give onto the [`parse`](fn@crate::parse) function, which returns
the struct [`ParsedArguments`](struct@parser::ParsedArguments) where you can access the arguments and flags.

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

*/
// Made modules in this program
mod builder;
/// Contains its own Error struct along with error types
pub mod errors;
mod help;
pub mod parser;

// Simplifying modularization within the API
pub use builder::arg::Arg;
pub use builder::context::Context;
pub use builder::flag::Flag;
pub use parser::parse;

// Use of the standard library
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
