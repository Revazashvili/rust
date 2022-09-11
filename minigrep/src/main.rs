mod lib;

use std::env;
use std::process;

use lib::{run, Config};

fn main() {
    let config = Config::new(env::args().collect())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}",err);
            process::exit(1)
        });

    if let Err(e) = run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}
