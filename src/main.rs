use rpstree::{self, cli};
use std::{env, process};

fn main() {
    let config = cli::Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    match rpstree::run(config) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
