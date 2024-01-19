//! A helper library to assist with mutating a struct based on the parameters
//! passed in command-line arguments.

#[cfg(test)]
mod test;

/// Exposes mutation of a struct representing valid command-line options for
/// each passed command-line argument.
pub trait ArgsExt: Iterator {
    /// Any required modifications to be performed before
    /// [with_args](`crate::ArgsExt::with_args`) is called.
    fn before_with_args(&mut self) {
        // do nothing
    }

    /// Performs the `do_while` argument for each item in `self`.
    fn with_args<T, F, E>(&mut self, default: &mut T, do_while: F) -> Result<(), E>
    where
        F: Fn(&mut Self, &mut T, <Self as Iterator>::Item) -> Result<(), E>,
    {
        self.before_with_args();

        loop {
            let arg = match self.next() {
                Some(arg) => arg,
                None => break,
            };

            do_while(self, default, arg)?;
        }

        Ok(())
    }
}

impl ArgsExt for std::env::Args {
    fn before_with_args(&mut self) {
        self.next(); // remove program call
    }
}

