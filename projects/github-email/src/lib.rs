pub use errors::{GithubError, Result};
pub use methods::{by_network_events::collect_network_events, by_user_events::collect_user_events, Authors};

mod errors;
mod methods;
