use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[cfg(feature = "octocrab")]
mod for_octocrab;
mod for_reqwest;
mod for_serde_json;

#[derive(Debug, Clone)]
pub enum GithubError {
    NetworkError(String),
    RuntimeError(String),
    UnknownError,
}

impl Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for GithubError {}

pub type Result<T = ()> = std::result::Result<T, GithubError>;
