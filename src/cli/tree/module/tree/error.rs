use bloock_storage::error::StorageError;
use thiserror::Error as ThisError;

use crate::cli::tree::{infrastructure::ports::SmtError, module::utils::error::UtilsError};


#[derive(ThisError,Debug)]
pub enum TreeError {
    #[error("The substate provided is incorrect as state_id is not present")]
    UtilsError(#[from] UtilsError),
    #[error(transparent)]
    SmtError(#[from] SmtError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
}