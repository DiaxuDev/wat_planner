mod fetch;
mod generate;

use crate::{
    cli::{fetch::FetchCommand, generate::GenerateCommand},
    error::Result,
};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Fetch schedule from wel.wat.edu.pl
    Fetch(FetchCommand),
    /// Generate schedule display
    Generate(GenerateCommand),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Fetch(fetch_command) => fetch_command.run(),
            Command::Generate(generate_command) => generate_command.run(),
        }
    }
}
