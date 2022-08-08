use std::env;
use std::{io,fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Could not read input.");
            println!("Here's the line: {}", line);
        },
        2 => {
            let path = &args[1];
            let source = fs::read_to_string(path).expect("No such file.");
            println!("Here's the source: {}", source);
        },
        _ => panic!("Please either provide a file path or no arguments"),
    };
}
