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

# fn main() {
let arg = Arg::new("Name", Type::Str, "This is the description", true, None);
println!("{:#?}", arg);
# }
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
use parsin::Context;

# fn main() {
let context = Context::new();
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
