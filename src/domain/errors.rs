use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZoomrsError {
    #[error("{}: already existing alias", .0)]
    AlreadyAdded(String),
    #[error("{}: no room with alias", .0)]
    NotPresent(String),
}
