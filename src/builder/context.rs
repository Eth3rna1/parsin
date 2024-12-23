use crate::Flag;
use crate::Type;
use crate::Arg;
use crate::errors::Error;
use crate::errors::ErrorKind;

/// [`Context`]
#[derive(Debug, Clone)]
pub struct Context {
    /// Defined aribtrary arguments
    pub(crate) args: Vec<Arg>,
    /// Defined arbitrary flags you want
    pub(crate) flags: Vec<Flag>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            flags: Vec::new(),
        }
    }

    /// getter
    pub fn args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    /// getter
    pub fn flags(&self) -> Vec<Flag> {
        self.flags.clone()
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

impl<const SIZE: usize> From<&(&[(&str, Type, &str, bool); SIZE], &[(&str, Type, &str, bool); SIZE])>  for Context{
    fn from(_tuple : &(&[(&str, Type, &str, bool); SIZE], &[(&str, Type, &str, bool); SIZE])) -> Self {
        let args: Vec<Arg> = _tuple.0.iter().map(|t| Arg::from(t)).collect();
        let flags: Vec<Flag> = _tuple.1.iter().map(|t| Flag::from(t)).collect();
        Self {
            args,
            flags
        }
    }
}

impl<const SIZE: usize> From<(&[(&str, Type, &str, bool); SIZE], &[(&str, Type, &str, bool); SIZE])>  for Context{
    fn from(_tuple : (&[(&str, Type, &str, bool); SIZE], &[(&str, Type, &str, bool); SIZE])) -> Self {
        let args: Vec<Arg> = _tuple.0.iter().map(|t| Arg::from(t)).collect();
        let flags: Vec<Flag> = _tuple.1.iter().map(|t| Flag::from(t)).collect();
        Self {
            args,
            flags
        }
    }
}
