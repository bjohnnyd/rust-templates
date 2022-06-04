mod cli;

use structopt::StructOpt;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    cli.set_logging();

    println!("Hello, world!");

    Ok(())
}
