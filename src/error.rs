#[derive(thiserror::Error, Debug)]
pub enum PlannerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("error parsing schedule")]
    Parse(#[from] crate::parser::Error),
    #[error("{0}")]
    Msg(&'static str),
}

pub type Result<T> = std::result::Result<T, PlannerError>;
