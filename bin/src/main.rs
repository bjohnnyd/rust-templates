#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms, dead_code, unused)]

mod cli;

use structopt::StructOpt;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    cli.set_logging();

    println!("Hello, world!");

    Ok(())
}
