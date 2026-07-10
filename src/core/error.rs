use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ThreeThoughtsError {
    #[error("No instructions of 3Thoughts: {0}")]
    NoInstructions(String),
}
