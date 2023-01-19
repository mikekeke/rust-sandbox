use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args() can accept only valid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // can do it like so coz don't need return value
    // unlike config parsing
    if let Err(_) = minigrep::run(config) {
        process::exit(1);
    }
}
