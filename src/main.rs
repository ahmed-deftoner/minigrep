use std::env;
use std::fs;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing file : {err}");
        process::exit(1);
    });
    println!("searching for {}",config.query);
    println!("in file {}",config.file_path);
     
    let content = fs::read_to_string(config.file_path)
    .expect("should have read string");

    println!("text:\n{content}");
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("few arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}