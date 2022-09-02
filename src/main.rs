use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];
    println!("searching for {}",query);
    println!("in file {}",filepath);
     
    let content = fs::read_to_string(filepath)
    .expect("should have read string");

    println!("text:\n{content}");
}
