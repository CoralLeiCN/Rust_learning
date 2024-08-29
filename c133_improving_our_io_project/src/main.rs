use std::env;
use std::process;

use c133_improving_our_io_project::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--

    if let Err(e) = c133_improving_our_io_project::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
