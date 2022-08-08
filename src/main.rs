use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());

    match args.len() {
        1 => println!("Scanning input!"),
        2 => println!("Scanning file!"),
        _ => panic!("Please either provide a file path or no arguments"),
    }
}
