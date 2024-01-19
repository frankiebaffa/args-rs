//! A helper library to assist with mutating a struct based on the parameters
//! passed in command-line arguments.

#[cfg(test)]
mod test;

/// Exposes mutation of a struct representing valid command-line options for
/// each passed command-line argument.
pub trait ArgsExt<Item: AsRef<str>>: Iterator<Item = Item> {
    /// Any required modifications to be performed before
    /// [with_args](`crate::ArgsExt::with_args`) is called.
    fn before_with_args(&mut self) {
        // do nothing
    }

    /// Performs the `do_while` argument for each item in `self`.
    fn with_args<T, F, E>(&mut self, default: &mut T, do_while: F) -> Result<(), E>
    where
        F: Fn(&mut Self, &mut T, &str) -> Result<(), E>,
    {
        self.before_with_args();

        loop {
            let arg = match self.next() {
                Some(arg) => arg,
                None => break,
            };

            let arg = arg.as_ref();

            if arg.starts_with("--") {
                let arg = &arg[2..];
                do_while(self, default, arg)?;
            } else if arg.starts_with("-") {
                for arg in arg[1..].chars() {
                    do_while(self, default, &arg.to_string())?;
                }
            } else {
                do_while(self, default, arg)?;
            }
        }

        Ok(())
    }
}

impl ArgsExt<String> for std::env::Args {
    fn before_with_args(&mut self) {
        self.next(); // remove program call
    }
}

