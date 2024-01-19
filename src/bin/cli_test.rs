use {
    args::ArgsExt,
    std::env::{ self, Args, },
};

struct Options {
    name: String,
    value: String,
    number: usize,
    verbose: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            name: String::new(),
            value: String::new(),
            number: 0,
            verbose: false,
        }
    }
}

impl Options {
    fn from_args(args: &mut Args, options: &mut Self, arg: &str) -> Result<(), String> {
        match arg {
            "n"|"name" => options.name = match args.next() {
                Some(arg) => arg,
                None => return Err(format!("Argument {arg} requires a value")),
            },
            "v"|"value" => options.value = match args.next() {
                Some(arg) => arg,
                None => return Err(format!("Argument {arg} requires a value")),
            },
            "u"|"number" => options.number = match args.next() {
                Some(arg) => match arg.parse::<usize>() {
                    Ok(arg) => arg,
                    Err(_) => return Err(format!("Argument {arg} requires a numeric value")),
                },
                None => return Err(format!("Argument {arg} requires a value")),
            },
            "b"|"verbose" => options.verbose = true,
            _ => return Err(format!("Unknown argument \"{arg}\"")),
        }

        Ok(())
    }
}

fn main() {
    let mut options = Options::default();

    env::args().with_args(&mut options, Options::from_args).unwrap();

    println!("Name:    {}", options.name);
    println!("Value:   {}", options.value);
    println!("Number:  {}", options.number);
    println!("Verbose: {}", options.verbose);
}
