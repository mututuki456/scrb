mod cli;
mod config;
mod commands;
mod markdown;
mod scrapbox;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::command().run();
    let config = config::load()?;
    let sid = config.sid.as_deref();

    match args {
        cli::Command::Md { project, page } => {
            commands::md::run(
                &project,
                &page,
                sid
            )?;
        }

        cli::Command::Raw { project, page } => {
            commands::raw::run(
                &project,
                &page,
                sid
            )?;
        }

        cli::Command::List { project, keyword } => {
            commands::list::run(
                &project,
                keyword.as_deref(),
                sid
            )?;
        }
    }

    Ok(())
}
