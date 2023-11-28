
pub trait SmtInterface<D>
where
    D: KeyValue + 'static,
{
    fn add_leaves(
        storage: D,
        root: Option<String>,
        messages: Vec<String>,
    ) -> Result<String, SmtError>;
    fn get_proof(storage: D, root: String, messages: Vec<String>) -> Result<Proof, SmtError>;
}

use bloock_storage::kv::KeyValue;
use thiserror::Error as ThisError;

use crate::cli::tree::module::tree::tree::Proof;
use crate::cli::tree::module::utils::error::UtilsError;
pub use bloock_smt::tree::SmtError as SmtLibraryError;

#[derive(ThisError, Debug)]
pub enum SmtError {
    #[error(transparent)]
    UtilsError(#[from] UtilsError),
    #[error(transparent)]
    SparseLibraryError(#[from] SmtLibraryError),
}
