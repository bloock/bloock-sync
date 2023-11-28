use thiserror::Error as ThisError;

#[derive(ThisError,Debug)]
pub enum UtilsError {
    #[error("The hex hash value {0} is malformed.")]
    HexError(String),
    #[error("The path provided is malformed.")]
    PathError(),
}
