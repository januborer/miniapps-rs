use minigrep::Config;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("ENV {}", env::var("IGNORE").expect("asdf"));

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args:{err}");

        process::exit(1);
    });

    // println!("The file is {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    };
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
