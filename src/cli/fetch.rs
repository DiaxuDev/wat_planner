use chrono::Datelike;
use yansi::Paint;

use crate::{cache, error::Result, fetcher::Term};

#[derive(clap::Args, Debug)]
pub struct FetchCommand {
    /// Whether to download schedule even if it exists locally
    #[arg(short, long, default_value = "false")]
    pub force: bool,
    /// WAT group identifier
    pub group: String,
    /// Which term to fetch the schedule for. If left empty it is detected automatically
    pub term: Option<Term>,
}

impl FetchCommand {
    pub fn run(&self) -> Result<()> {
        let term = self
            .term
            .unwrap_or_else(|| Term::from_month(chrono::Local::now().month()));

        let path = cache::file_path(&self.group, term);

        if !self.force && path.exists() {
            eprintln!("{}", "There already is saved schedule for this group and term. Rerun with --force to override.".yellow());
            return Ok(());
        }

        cache::save(&self.group, term, &path)?;
        Ok(())
    }
}
