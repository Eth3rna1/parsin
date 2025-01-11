mod builder;
pub(crate) mod funcs;

use crate::errors::Error;
use crate::get_env_args;
use crate::Context;

pub use builder::ParsedArguments;
pub use builder::Value;

use std::sync::LazyLock;

static ARGUMENTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    // getting rid of the argument containing the file name because
    // it will only cause trouble in the long run
    let bind = get_env_args();
    match bind.len() {
        1 => Vec::new(),
        2 | _ => bind[1..].to_vec(),
    }
});

fn _unwrap_or_throw<R>(variant: Result<R, Error>, ctx: &Context) -> R {
    use crate::help::send_help_and_exit;

    if let Err(error) = variant {
        send_help_and_exit(ctx, &error);
    }
    variant.unwrap()
}

/// # Main Function to be executed when parsin the CLI arguments
pub fn parse(__ctx: &Context) -> ParsedArguments {
    {
        // Checking for the `--help` flag.
        // Looking for an early return
        _unwrap_or_throw(funcs::_look_for_help(&ARGUMENTS), __ctx);
    }
    let mut __parsed = ParsedArguments::new();
    let mut __args: Vec<String> = Vec::new();
    let mut __flags_and_values: Vec<String> = Vec::new();
    {
        (__args, __flags_and_values) = funcs::_arguments_and_options(&ARGUMENTS, __ctx);
    }
    {
        _unwrap_or_throw(funcs::_check_mandatory_args(&__args, __ctx), __ctx);
    }
    {
        _unwrap_or_throw(funcs::_check_mandatory_flags(&ARGUMENTS, __ctx), __ctx);
    }
    {
        _unwrap_or_throw(
            funcs::_assign_arguments(&mut __parsed, &__args, __ctx),
            __ctx,
        );
    }
    {
        _unwrap_or_throw(
            funcs::_assign_options(&mut __parsed, &__flags_and_values, __ctx),
            __ctx,
        );
    }
    __parsed
}
