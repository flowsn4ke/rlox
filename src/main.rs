use std::env;
use std::{io,fs};
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            run_prompt();
        },
        2 => {
            let path = &args[1];
            run_file(path);
        },
        _ => panic!("Please either provide a file path or no arguments"),
    };
}

fn run(source: String) {
    println!("{}", source);
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("No such file.");
    run(source);
}

fn run_prompt() {
    println!("Start typing your commands...");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut source = String::new();

        io::stdin()
            .read_line(&mut source)
            .expect("Could not read input.");
   
        source = String::from(source.trim_end());

        run(source);
    }
}
