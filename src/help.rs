/*
    Help with the UI That is displayed
*/
use crate::errors::Error;
use crate::errors::ErrorKind;
use crate::get_env_args;
use crate::Context;

use std::process::exit;

/// Constructs the error message, stderrs, and exits with status 1
///
/// | Type                                | Name    | Description                                  |
/// |-------------------------------------|---------|----------------------------------------------|
/// | &[`Context`](struct@crate::Context) | context | A reference to the context of the CLI parser |
/// |-------------------------------------|---------|----------------------------------------------|
/// | &[`Error`](struct@crate::errors::Error) | error | A reference to the error returned to throw |
pub fn send_help_and_exit(context: &Context, error: &Error) -> ! {
    // WantsHelp error signifies that the client raised the `--help`
    // flag, thus the program will throw an advanced error for the client
    if error.kind() == ErrorKind::WantsHelp {
        // function exits on its own,
        // so it's not necessary to manually exit
        send_advanced_help_and_exit(context);
        // exit(1)  <<<    redundant code
    }
    let error_msg = format!(
        r#"{}

For more information, try passing `--help`."#,
        error
    );
    eprintln!("{}", error_msg);
    exit(1);
}

/// Constructs a more advanced, descriptive version of the help message, stderrs,
/// and exits
/// | Type   | Name | Description |
/// |--------------------------------------|
/// | &[`Context`] | context | A reference to the context fo the CLI parser |
pub fn send_advanced_help_and_exit(context: &Context) -> ! {
    let name: &String = &{
        let bind = std::path::PathBuf::from(&get_env_args()[0]);
        bind.file_name().unwrap().to_str().unwrap().to_string()
    };
    let mut error_msg: String = String::new();
    error_msg += "Usage: ";
    error_msg += &name;
    error_msg += " [OPTIONS] ";
    if !context.args.is_empty() {
        let mut lines: Vec<String> = Vec::new();
        for arg in context.args.iter() {
            lines.push(format!("<{}>", arg.name.to_uppercase()));
        }
        error_msg += &lines.join(" ");
        error_msg += r#"

ARGUMENTS
--------
"#;
        let mut lines: Vec<String> = Vec::new();
        for arg in context.args.iter() {
            let mut buffer = format!(
                "{:>15}   {}",
                format!("<{}>", arg.name.to_uppercase()),
                arg.help
            );
            if let Some(def) = &arg.default {
                buffer += &format!(" [DEFAULT={}]", def);
            }
            lines.push(buffer);
        }
        error_msg += &lines.join("\n");
    }
    // Options are ALWAYS going to display because
    // the `--help` flag will always exist
    error_msg += r#"

OPTIONS
------
"#;
    let mut lines: Vec<String> = Vec::new();
    for flag in context.flags.iter() {
        let mut buffer = format!("{:>15}   {}", flag.name, flag.help);
        if let Some(def) = &flag.default {
            buffer += &format!(" [DEFAULT={}]", def);
        }
        lines.push(buffer);
    }
    lines.push(format!("{:>15}   Displays this message", "--help"));
    error_msg += &lines.join("\n");
    eprintln!("{}", error_msg);
    exit(1);
}
