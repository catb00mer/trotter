#![doc = include_str!("../readme.md")]

mod actor;
pub mod error;
mod response;
mod status;
mod user_agent;
mod utils;

pub use actor::{trot, Actor};
pub use response::Response;
pub use status::Status;
pub use user_agent::UserAgent;
