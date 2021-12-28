#[derive(Debug, Clone)]
pub enum GithubError {
    NetworkError(String),
    UnknownError,
}

pub type Result<T = ()> = std::result::Result<T, GithubError>;

mod for_reqwest;
