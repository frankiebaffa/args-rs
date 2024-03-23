//! A helper library to assist with mutating a struct based on the parameters
//! passed in command-line arguments.

use std::{
    env::args,
    io::{ Error, ErrorKind, Result, },
    vec::IntoIter,
};

pub enum ArgumentType {
    Short(String),
    Long(String),
}

pub enum OptionType {
    Argument(ArgumentType),
    Value(String),
}

pub struct Argument {
    option_type: OptionType,
    position: usize,
    max_position: usize,
}

impl Argument {
    pub fn qualifier(&self) -> &str {
        match &self.option_type() {
            OptionType::Argument(arg) => match arg {
                ArgumentType::Short(q) => q,
                ArgumentType::Long(q) => q,
            },
            OptionType::Value(q) => q,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.option_type() {
            OptionType::Argument(arg) => match arg {
                ArgumentType::Short(q) => format!("-{q}"),
                ArgumentType::Long(q) => format!("--{q}"),
            },
            OptionType::Value(s) => s.to_owned(),
        }
    }

    pub fn option_type(&self) -> &OptionType {
        &self.option_type
    }

    pub fn is_short(&self) -> bool {
        match self.option_type() {
            OptionType::Argument(ArgumentType::Short(_)) => true,
            _ => false,
        }
    }

    pub fn is_long(&self) -> bool {
        match self.option_type() {
            OptionType::Argument(ArgumentType::Long(_)) => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self.option_type() {
            OptionType::Value(_) => true,
            _ => false,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn max_position(&self) -> usize {
        self.max_position
    }

    pub fn from_last(&self) -> usize {
        self.max_position - self.position
    }

    pub fn is_n_from_last(&self, n: usize) -> bool {
        self.from_last() == n
    }

    pub fn is_last(&self) -> bool {
        self.is_n_from_last(0)
    }

    pub fn from_first(&self) -> usize {
        self.position
    }

    pub fn is_n_from_first(&self, n: usize) -> bool {
        self.from_first() == n
    }

    pub fn is_first(&self) -> bool {
        self.is_n_from_first(0)
    }
}

pub struct Arguments(IntoIter<Argument>);
impl Arguments {
    pub fn next(&mut self) -> Option<Argument> {
        self.0.next()
    }

    pub fn enforce_next_value(&mut self, prev: Argument) -> Result<String> {
        match self.next() {
            Some(i) => match i.option_type() {
                OptionType::Value(_) => {
                    Ok(i.qualifier().to_owned())
                },
                _ => return Err(Error::new(
                    ErrorKind::Other,
                    format!("{} requires a value.", prev.to_string()),
                )),
            },
            None => return Err(Error::new(
                ErrorKind::Other,
                format!("{} requires a value.", prev.to_string()),
            )),
        }
    }

    pub fn with_args<T, F>(default: &mut T, do_while: F) -> Result<()>
    where
        F: Fn(&mut Arguments, &mut T, Argument) -> Result<()>,
    {
        let mut std_args = args();
        std_args.next(); // dump first

        let mut std_args = args();
        std_args.next();
        let args = std_args
            .map(|arg| {
                if arg.starts_with("--") {
                    let arg = &arg[2..];
                    vec![OptionType::Argument(ArgumentType::Long(arg.to_owned()))]
                } else if arg.starts_with("-") {
                    arg[1..].chars().map(|arg| {
                        OptionType::Argument(ArgumentType::Short(arg.to_string()))
                    }).collect::<Vec<OptionType>>()
                } else {
                    vec![OptionType::Value(arg)]
                }
            })
            .flatten()
            .collect::<Vec<OptionType>>();
        let max = args.len() - 1;

        let mut args = Arguments(args.into_iter().enumerate()
            .map(|(idx, arg)| Argument {
                option_type: arg,
                position: idx,
                max_position: max,
            })
            .collect::<Vec<Argument>>()
            .into_iter()
        );

        while let Some(arg) = args.next() {
            do_while(&mut args, default, arg)?;
        }
        Ok(())
    }
}
