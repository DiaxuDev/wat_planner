use std::{env, io::Write, path::PathBuf};

use yansi::Paint;

use crate::{
    error::{PlannerError, Result},
    fetcher::{Term, fetch_schedule},
    parser::parse,
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
}

#[derive(clap::Args, Debug)]
struct FetchCommand {
    /// Whether to download schedule even if it exists locally
    #[arg(short, long, default_value = "false")]
    pub force: bool,
    /// WAT group identifier
    pub group: String,
    /// Which term to fetch the schedule for. If left empty it is detected automatically
    pub term: Option<Term>,
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
        }
    }
}

impl FetchCommand {
    pub fn run(&self) -> Result<()> {
        let term = self.term.unwrap_or_else(Term::detect);

        let path = xdg_state_home().join(format!("{}-{term}.json", self.group));

        if !self.force && path.exists() {
            eprintln!("{}", "There already is saved schedule for this group and term. Rerun with --force to override.".yellow());
            return Ok(());
        }

        println!("Fetching...");
        let raw = fetch_schedule(term, &self.group)?;

        println!("Parsing....");
        let data = parse(&raw)?;

        println!("Saving...");
        std::fs::create_dir_all(
            path.parent()
                .ok_or(PlannerError::Msg("invalid file path"))?,
        )?;
        let mut file = std::fs::File::create(&path)?;
        serde_json::to_writer(&file, &data)?;
        file.flush()?;

        println!("Saved to {}", path.display());

        Ok(())
    }
}

fn xdg_state_home() -> PathBuf {
    env::var_os("XDG_STATE_HOME")
        .filter(|x| !x.is_empty())
        .map_or_else(
            || {
                env::home_dir()
                    .expect("an XDG environment should have a home directory")
                    .join(".local/state")
            },
            PathBuf::from,
        )
        .join(env!("CARGO_PKG_NAME"))
}
