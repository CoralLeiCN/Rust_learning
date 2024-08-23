use std::env;

use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        // no longer get all the extra output
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
