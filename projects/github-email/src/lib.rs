#[doc = include_str!("../Readme.md")]
pub use self::{
    authors::{AuthorQuery, Authors, CommitAuthor},
    errors::{GithubError, Result},
    methods::{
        by_network_events::collect_network_events, by_repo_events::collect_repo_events, by_user_events::collect_user_events,
        parse_queries,
    },
};

mod authors;
mod errors;
mod methods;
