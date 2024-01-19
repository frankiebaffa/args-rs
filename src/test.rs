use {
    crate::ArgsExt,
    std::str::Split,
};

struct Options {
    name: String,
    value: String,
}

impl Options {
    fn extract(
        args: &mut Split<'_, &str>,
        options: &mut Self,
        arg: &str
    ) -> Result<(), String> {
        match arg {
            "-n"|"--name" => options.name = match args.next() {
                Some(name) => name.to_string(),
                None => return Err("Argument --name requires a value".to_string().into()),
            },
            "-v"|"--value" => options.value = match args.next() {
                Some(value) => value.to_string(),
                None => return Err("Argument --value requires a value".to_string().into()),
            },
            _ => return Err(format!("Unknown argument \"{arg}\"")),
        }

        Ok(())
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }
}

impl<'a> ArgsExt for Split<'a, &str> {}

#[test]
fn test() {
    let args_s = "-n Something -v TheValue";
    let mut args = args_s.split(" ");
    let mut options = Options::default();

    args.with_args(&mut options, Options::extract).unwrap();

    assert_eq!("Something", options.name);

    assert_eq!("TheValue", options.value);
}

#[test]
#[should_panic]
fn panic() {
    let args_s = "-n Something -v";
    let mut args = args_s.split(" ");
    let mut options = Options::default();

    args.with_args(&mut options, Options::extract).unwrap();
}
