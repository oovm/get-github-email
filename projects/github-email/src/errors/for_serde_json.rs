use octocrab::Error;

use crate::GithubError;

impl From<Error> for GithubError {
    fn from(error: Error) -> Self {
        GithubError::NetworkError(error.to_string())
    }
}
