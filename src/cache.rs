use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    data::Data,
    error::{PlannerError, Result},
    fetcher::{Term, fetch},
    parser::parse,
};

pub fn save(group: &str, term: Term, path: &Path) -> Result<Data> {
    println!("Fetching...");
    let raw = fetch(term, group)?;

    println!("Parsing....");
    let data = parse(&raw)?;

    println!("Saving...");
    std::fs::create_dir_all(
        path.parent()
            .ok_or(PlannerError::Msg("invalid file path"))?,
    )?;
    let mut file = std::fs::File::create(path)?;
    serde_json::to_writer(&file, &data)?;
    file.flush()?;

    println!("Saved to {}", path.display());

    Ok(data)
}

fn read(path: &Path) -> Result<Data> {
    let raw = std::fs::read_to_string(path)?;
    serde_json::from_str(&raw).map_err(PlannerError::Serde)
}

pub fn read_or_save(group: &str, term: Term) -> Result<Data> {
    let path = file_path(group, term);
    match read(&path) {
        Err(PlannerError::Io(error)) if matches!(error.kind(), std::io::ErrorKind::NotFound) => {
            save(group, term, &path)
        }
        other => other,
    }
}

pub fn file_path(group: &str, term: Term) -> PathBuf {
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
        .join(format!("{}/{group}-{term}.json", env!("CARGO_PKG_NAME")))
}
