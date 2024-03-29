extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let mut stderr = std::io::stderr();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Problem parsing the arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        writeln!(&mut stderr, "Application error: {}", e)
            .expect("Could not write to stderr");
        process::exit(1);
    };
}
