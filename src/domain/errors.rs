use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZoomrsError {
    #[error("{}: already existing alias", .0)]
    AlreadyAdded(String),
}