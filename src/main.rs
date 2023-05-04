use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("Searching for {:?}", config.query);
    println!("In the file {:?}", config.file_path);
    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    })
}
