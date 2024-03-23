use {
    args::Arguments,
    std::io::Result,
};

fn main() {
    let mut x = String::new();
    Arguments::with_args(&mut x, |_, _, c| {
        println!("QUALIFIER: {}", c.qualifier());
        println!("POSITION:  {}", c.position());
        println!("MAX_POS:   {}", c.max_position());
        Result::Ok(())
    }).unwrap();
}
