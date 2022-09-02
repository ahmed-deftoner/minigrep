use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let (query,filepath) = parse_config(&args);
    println!("searching for {}",query);
    println!("in file {}",filepath);
     
    let content = fs::read_to_string(filepath)
    .expect("should have read string");

    println!("text:\n{content}");
}

fn parse_config(args : &[String]) -> (&str,&str) {
    let query = &args[1];
    let filepath = &args[2];

    (query,filepath)
}