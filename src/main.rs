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

struct Config {
    query: String,
    file_path: String
}

fn parse_config(args : &[String]) -> Config {
    let query = args[1].clone();
    let filepath = args[2].clone();

    Config { query, file_path}
}