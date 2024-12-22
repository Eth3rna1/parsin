/*
    Help with the UI That is displayed
*/
use crate::errors::Error;
use crate::errors::ErrorKind;
use crate::Context;
use crate::Type;

use std::convert::Into;
use std::process::exit;

pub fn send_help_and_exit(context: &Context, error: &Error) -> ! {
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

pub fn send_advanced_help_and_exit(context: &Context) -> ! {
    let mut error_msg: String = String::new();
    error_msg += "Usage: [OPTIONS] ";
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
            lines.push(format!(
                "{:>15}   {}",
                format!("<{}>", arg.name.to_uppercase()),
                arg.help
            ));
        }
        error_msg += &lines.join("\n");
    }
    error_msg += r#"

OPTIONS
------
"#;
    let mut lines: Vec<String> = Vec::new();
    for flag in context.flags.iter() {
        lines.push(format!("{:>15}   {}", flag.name, flag.help));
    }
    lines.push(format!("{:>15}   Display this message", "--help"));
    error_msg += &lines.join("\n");
    eprintln!("{}", error_msg);
    exit(1);
}