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

    if let Err(error) = opts.command.run(api) {
        if let Some(mut t) = term::stderr() {
            let _ = t.attr(term::Attr::Bold);
            let _ = t.bg(term::color::RED);
            let _ = write!(t, " ERROR ");
            let _ = t.reset();
        } else {
            eprint!(" ERROR ");
        }

        eprintln!(" {error}");
    }

    Ok(())
}
