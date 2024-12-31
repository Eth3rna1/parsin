use crate::errors::Error;
use crate::errors::ErrorKind;
use crate::parser::ParsedArguments;
use crate::parser::Value;
use crate::Context;
use crate::Type;

pub(crate) fn _look_for_help(args: &[String]) -> Result<(), Error> {
    if args.contains(&"--help".to_string()) {
        return Err(Error::new(ErrorKind::WantsHelp, String::new()));
    }
    Ok(())
}

fn __find_and_remove<T: PartialEq>(list: &mut Vec<T>, item: &T) -> Option<T> {
    // if item not in the list or is empty
    let llen = list.len();
    if !list.contains(item) || llen == 0 {
        return None;
    }
    // this is an anonymous function, it does NOT return
    // for the outter function
    let index: usize = (|l: &[T], i: &T| {
        if l.len() == 1 {
            return 0;
        }
        let mut bind: usize = 0;
        for idx in 0..llen {
            if list[idx] == *i {
                bind = idx;
                break;
            }
        }
        bind
    })(list, item);
    Some(list.remove(index))
}

pub(crate) fn _arguments_and_options(args: &[String], ctx: &Context) -> (Vec<String>, Vec<String>) {
    let mut __args: Vec<String> = args.to_vec();
    let mut __f_v: Vec<String> = Vec::new();
    let mut __pointer: usize = 0;
    let a_len = args.len();
    while __pointer < a_len {
        if let Some(flag) = ctx.get_flag(&args[__pointer]) {
            match flag.r#type {
                Type::Bool => {
                    let bind = __find_and_remove(&mut __args, &args[__pointer]).unwrap();
                    __f_v.push(bind);
                    __pointer += 1;
                }
                // type of flag that must contain a value
                Type::Str | Type::Int => {
                    let bind = __find_and_remove(&mut __args, &args[__pointer]).unwrap();
                    __f_v.push(bind);
                    if __pointer + 1 < a_len {
                        let bind = __find_and_remove(&mut __args, &args[__pointer + 1]).unwrap();
                        __f_v.push(bind);
                        if &args[__pointer + 1] == "--" && __pointer + 2 < a_len {
                            let bind =
                                __find_and_remove(&mut __args, &args[__pointer + 2]).unwrap();
                            __f_v.push(bind);
                            __pointer += 3;
                            continue;
                        }
                    }
                    __pointer += 2;
                }
            }
            // end of match statement
            // trying to avoid using else statements
            continue;
        }
        // ELSE:
        __pointer += 1;
    }
    (__args, __f_v)
}

pub(crate) fn _check_mandatory_args(args: &[String], ctx: &Context) -> Result<(), Error> {
    let a_len = args.len();
    let c_len = ctx.args.len();
    if a_len >= c_len {
        return Ok(());
    }
    // meaning theres no sufficient args
    // to assign to every argument defined
    // within Context
    /*
        CONTEXT ARGS: [1, 2, 3, 4, 5]
          ARGS GIVEN: [1, 2, 3, 4, _]
                                  ^^^
                                   |
                             Missing Argument
    */
    for i in a_len..c_len {
        let arg = &ctx.args[i];
        if arg.is_mandatory {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                format!("Missing argument `<{}>`", arg.name.to_uppercase()),
            ));
        }
    }
    // all missing arguments were justified
    Ok(())
}

pub(crate) fn _check_mandatory_flags(args: &[String], ctx: &Context) -> Result<(), Error> {
    for flag in ctx.flags.iter() {
        if flag.is_mandatory && !args.contains(&flag.name) {
            return Err(Error::new(
                ErrorKind::MissingFlag,
                format!("Missing flag `<{}>`", flag.name.to_uppercase()),
            ));
        }
    }
    Ok(())
}

pub(crate) fn _assign_arguments(
    parsed: &mut ParsedArguments,
    args: &[String],
    ctx: &Context,
) -> Result<(), Error> {
    let a_len = args.len();
    let c_len = ctx.args.len();
    // meaning not every argument
    // defined in Context is not
    // going to get populated
    if a_len < c_len {
        // pointer is going to be used
        // to continue where it left off
        let mut ptr: usize = 0;
        for i in 0..a_len {
            let assigned_arg = &ctx.args[i];
            match assigned_arg.r#type {
                Type::Str => {
                    let value = Value::Str(args[i].clone());
                    parsed.arguments.insert(assigned_arg.name.clone(), value);
                }
                Type::Bool => {
                    let value = Value::Bool(true);
                    parsed.arguments.insert(assigned_arg.name.clone(), value);
                }
                Type::Int => {
                    let int_parsed = args[i].parse::<i32>();
                    if let Err(error) = int_parsed {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Ref: `<{}>`, {}", assigned_arg.name.to_uppercase(), error),
                        ));
                    }
                    let value = Value::Int(int_parsed.unwrap());
                    parsed.arguments.insert(assigned_arg.name.clone(), value);
                }
            }
            ptr += 1;
        }
        // Continuing where pointer left off
        // No argument was provided for the defined
        // arguments within context beyond this point
        for i in ptr..c_len {
            let assigned_arg = &ctx.args[i];
            // parsing default value
            if let Some(str_val) = &assigned_arg.default {
                match assigned_arg.r#type {
                    Type::Str => {
                        let value = Value::Str(str_val.to_string());
                        parsed.arguments.insert(assigned_arg.name.clone(), value);
                    }
                    Type::Bool => {
                        let bind = str_val.trim();
                        let bind = bind.to_lowercase();
                        match bind.as_str() {
                            "true" => {
                                let value = Value::Bool(true);
                                parsed.arguments.insert(assigned_arg.name.clone(), value);
                            }
                            "false" => {
                                let value = Value::Bool(false);
                                parsed.arguments.insert(assigned_arg.name.clone(), value);
                            }
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    format!("Unparsable default value {:?}", bind),
                                ))
                            }
                        }
                    }
                    Type::Int => {
                        let int_parsed = args[i].parse::<i32>();
                        if let Err(error) = int_parsed {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Ref: `<{}>`, {}", assigned_arg.name.to_uppercase(), error),
                            ));
                        }
                        let value = Value::Int(int_parsed.unwrap());
                        parsed.arguments.insert(assigned_arg.name.clone(), value);
                    }
                }
                // avoid using an else statement
                continue;
            }
            parsed
                .arguments
                .insert(assigned_arg.name.clone(), Value::Null);
        }
        return Ok(());
    }
    // ELSE: arg length is greater than
    // or equal to arguments length within Context
    // Meaning every argument will get assigned
    for i in 0..c_len {
        let assigned_arg = &ctx.args[i];
        match assigned_arg.r#type {
            Type::Str => {
                let value = Value::Str(args[i].clone());
                parsed.arguments.insert(assigned_arg.name.clone(), value);
            }
            Type::Bool => {
                let value = Value::Bool(true);
                parsed.arguments.insert(assigned_arg.name.clone(), value);
            }
            Type::Int => {
                let int_parsed = args[i].parse::<i32>();
                if let Err(error) = int_parsed {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Ref: `<{}>`, {}", assigned_arg.name.to_uppercase(), error),
                    ));
                }
                let value = Value::Int(int_parsed.unwrap());
                parsed.arguments.insert(assigned_arg.name.clone(), value);
            }
        }
    }
    Ok(())
}

fn __assign_uncovered_flags(
    parsed: &mut ParsedArguments,
    ctx: &Context,
    __covered_flags: &[&str],
) -> Result<(), Error> {
    // making use of the default value
    for flag in ctx.flags.iter() {
        if __covered_flags.contains(&flag.name.as_str()) {
            continue;
        }
        if let Some(def) = &flag.default {
            match flag.r#type {
                Type::Str => {
                    let value = Value::Str(def.to_owned());
                    parsed.flags.insert(flag.name.clone(), value);
                }
                Type::Bool => {
                    let bind = def.trim();
                    let bind = bind.to_lowercase();
                    match bind.as_str() {
                        "true" => {
                            let value = Value::Bool(true);
                            parsed.flags.insert(flag.name.clone(), value);
                        }
                        "false" => {
                            let value = Value::Bool(true);
                            parsed.flags.insert(flag.name.clone(), value);
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Ref: `{}`, unparsable default value", &flag.name),
                            ));
                        }
                    }
                }
                Type::Int => {
                    let int_result = def.parse::<i32>();
                    if int_result.is_err() {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Ref: `{}`, unparsable default value", &flag.name),
                        ));
                    }
                    let value = Value::Int(int_result.unwrap());
                    parsed.flags.insert(flag.name.clone(), value);
                }
            }
        } else {
            parsed.flags.insert(flag.name.clone(), Value::Null);
        }
    }
    Ok(())
}

pub(crate) fn _assign_options(
    parsed: &mut ParsedArguments,
    fv: &[String],
    ctx: &Context,
) -> Result<(), Error> {
    let mut __pointer: usize = 0;
    let mut __covered_flags: Vec<&str> = Vec::new();
    let fv_len = fv.len();
    if fv_len == 0 {
        __assign_uncovered_flags(parsed, ctx, &__covered_flags)?;
    }
    while __pointer < fv_len {
        if let Some(flag) = ctx.get_flag(&fv[__pointer]) {
            let bind = &fv[__pointer];
            __covered_flags.push(bind);
            match flag.r#type {
                Type::Str => {
                    if __pointer + 1 >= fv_len {
                        return Err(Error::new(
                            ErrorKind::MissingValue,
                            format!("Missing value for `{}`.", flag.name),
                        ));
                    }
                    // means value is a defined flag
                    if ctx.contains_flag(&fv[__pointer + 1]) {
                        let next_iter = &fv[__pointer + 1];
                        return Err(Error::new(
                            ErrorKind::MissingValue,
                            format!(
                                "Missing value for `{}`. \
if you want to pass `{}` as the value, try `-- {}`.",
                                flag.name, next_iter, next_iter
                            ),
                        ));
                    }
                    // if next iteration's value is "--"
                    if &fv[__pointer + 1] == "--" {
                        // Lets say the next iteration was "--", but no value was provided after
                        if __pointer + 2 >= fv_len {
                            return Err(Error::new(
                                ErrorKind::MissingValue,
                                format!("Missing value for `{}`", &flag.name),
                            ));
                        }
                        let value = Value::Str(fv[__pointer + 2].clone());
                        parsed.flags.insert(flag.name, value);
                        __pointer += 3;
                        continue;
                    }
                    let value = Value::Str(fv[__pointer + 1].clone());
                    parsed.flags.insert(flag.name, value);
                    __pointer += 2;
                }
                Type::Bool => {
                    let value = Value::Bool(true);
                    parsed.flags.insert(flag.name, value);
                    __pointer += 1;
                }
                Type::Int => {
                    if __pointer + 1 >= fv_len {
                        return Err(Error::new(
                            ErrorKind::MissingValue,
                            format!("Missing value for `{}`.", flag.name),
                        ));
                    }
                    // means value is a defined flag
                    if ctx.contains_flag(&fv[__pointer + 1]) {
                        let next_iter = &fv[__pointer + 1];
                        return Err(Error::new(
                            ErrorKind::MissingValue,
                            format!(
                                "Missing value for `{}`. \
if you want to pass `{}` as the value, try `-- {}`.",
                                flag.name, next_iter, next_iter
                            ),
                        ));
                    }
                    if &fv[__pointer + 1] == "--" {
                        if __pointer + 2 >= fv_len {
                            return Err(Error::new(
                                ErrorKind::MissingValue,
                                format!("Missing value for `{}`", &flag.name),
                            ));
                        }
                        let int_result = fv[__pointer + 2].parse::<i32>();
                        if let Err(error) = int_result {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Ref: `{}`, {}", &flag.name, error),
                            ));
                        }
                        let value = Value::Int(int_result.unwrap());
                        parsed.flags.insert(flag.name, value);
                        __pointer += 3;
                        continue;
                    }
                    let int_result = fv[__pointer + 1].parse::<i32>();
                    if let Err(error) = int_result {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Ref: `{}`, {}", &flag.name, error),
                        ));
                    }
                    let value = Value::Int(int_result.unwrap());
                    parsed.flags.insert(flag.name, value);
                    __pointer += 2;
                }
            }
        }
    }
    __assign_uncovered_flags(parsed, ctx, &__covered_flags)?;
    Ok(())
}
