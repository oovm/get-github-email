#[derive(Debug, Clone)]
pub enum GithubError {
    NetworkError(String),
    RuntimeError(String),
    UnknownError,
}

pub type Result<T = ()> = std::result::Result<T, GithubError>;

mod for_octocrab;
mod for_reqwest;
mod for_serde_json;
