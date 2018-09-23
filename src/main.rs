use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = readSource(filename).unwrap();

    let parsed = ast::parse(&mut contents.chars());

    println!("{:?}", contents);
    println!("{:?}", parsed);

    let code = parsed.unwrap_or(Vec::new());

    println!("{:?}", code);
    ast::executeWithVec(&code, &mut vec![0; 30000], 0);
}

fn readSource(filename: &str) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer);

    Ok(buffer)
}
