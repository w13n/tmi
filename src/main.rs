use clap::Parser;

use tmi::{cli, run};

fn main() {
    let result = run(cli::CArgs::parse());
    if let Err(error) = result {
        eprintln!("{}", error);
    }
}
