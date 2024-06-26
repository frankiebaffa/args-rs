//! A helper library to assist with mutating a struct based on the parameters
//! passed in command-line arguments.

use std::{
    env::args,
    fmt::{ Display, Formatter, Result as FmtResult, },
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

pub fn err<T, Str: AsRef<str>>(message: Str) -> Result<T> {
    Err(Error::new(ErrorKind::Other, message.as_ref().to_owned()))
}

impl Display for Argument {
    fn fmt(&self, fmtr: &mut Formatter<'_>) -> FmtResult {
        match &self.option_type() {
            OptionType::Argument(arg) => match arg {
                ArgumentType::Short(q) => fmtr.write_fmt(format_args!("-{q}")),
                ArgumentType::Long(q) => fmtr.write_fmt(format_args!("--{q}")),
            },
            OptionType::Value(s) => fmtr.write_str(s),
        }
    }
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

    pub fn option_type(&self) -> &OptionType {
        &self.option_type
    }

    pub fn is_short(&self) -> bool {
        matches!(self.option_type(), OptionType::Argument(ArgumentType::Short(_)))
    }

    pub fn is_long(&self) -> bool {
        matches!(self.option_type(), OptionType::Argument(ArgumentType::Long(_)))
    }

    pub fn is_value(&self) -> bool {
        matches!(self.option_type(), OptionType::Value(_))
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn max_position(&self) -> usize {
        self.max_position
    }

    pub fn from_last(&self) -> usize {
        (self.max_position - 1) - self.position
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
    pub fn next_arg(&mut self) -> Option<Argument> {
        self.0.next()
    }

    pub fn enforce_next_value(&mut self, prev: &Argument) -> Result<String> {
        match self.next_arg() {
            Some(i) => match i.option_type() {
                OptionType::Value(_) => {
                    Ok(i.qualifier().to_owned())
                },
                _ => err(format!("{} requires a value.", prev)),
            },
            None => err(format!("{} requires a value.", prev)),
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
            .flat_map(|arg| {
                if let Some(arg) = arg.strip_prefix("--") {
                    vec![OptionType::Argument(ArgumentType::Long(arg.to_owned()))]
                } else if let Some(arg) = arg.strip_prefix('-') {
                    arg.chars().map(|arg| {
                        OptionType::Argument(ArgumentType::Short(arg.to_string()))
                    }).collect::<Vec<OptionType>>()
                } else {
                    vec![OptionType::Value(arg)]
                }
            })
            .collect::<Vec<OptionType>>();

        let max = args.len();

        let mut args = Arguments(args.into_iter().enumerate()
            .map(|(idx, arg)| Argument {
                option_type: arg,
                position: idx,
                max_position: max,
            })
            .collect::<Vec<Argument>>()
            .into_iter()
        );

        while let Some(arg) = args.next_arg() {
            do_while(&mut args, default, arg)?;
        }
        Ok(())
    }
}
