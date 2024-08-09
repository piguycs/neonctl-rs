mod api;
mod cli;
mod commands;
mod prelude;
mod region;
mod style;
mod table;

use api::Api;
use clap::Parser;
use cli::Cli;

use crate::prelude::*;

fn main() -> Result<()> {
    let api = Api::try_new()?;

    let opts = Cli::parse();

    opts.command.run(api)?;

    Ok(())
}
