use std::fs::File;
use std::io::Read;
use std::{env, process};

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no file path provided");
        process::exit(1);
    }

    let file_path = &args[1];

    let mut text = String::new();
    let file = File::open(file_path);

    if file.is_err() {
        eprintln!("file {} not found", file_path);
        process::exit(1);
    }
    let res = file.unwrap().read_to_string(&mut text);
    if res.is_err() {
        eprintln!("unable to read file {}", file_path);
        process::exit(1);
    }

    let tokens = parser::parse_string(&text);
    dbg!(tokens);


}
