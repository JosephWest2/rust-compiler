use std::fs::File;
use std::io::Read;
use std::env;

mod lexer;
mod parser;
mod emitter;

use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no file path provided");
        return;
    }

    let file_path = &args[1];

    let mut text = String::new();
    let file = File::open(file_path);

    if file.is_err() {
        eprintln!("file {} not found", file_path);
        return;
    }
    let res = file.unwrap().read_to_string(&mut text);
    if res.is_err() {
        eprintln!("unable to read file {}", file_path);
        return;
    }

    let mut parser = Parser::new(&text);
    parser.run();
}
