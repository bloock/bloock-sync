use std::marker::PhantomData;
use std::sync::Arc;

use bloock_storage::kv::KvConnect;

use crate::cli::tree::infrastructure::ports::SmtInterface;

use super::error::TreeError;
use super::ports::TreeRepository;
use super::tree::Proof;

pub struct TreeRepositoryImpl<K: KvConnect, SMT: SmtInterface<K::Client>>
    where
        K::Client: 'static {
    pub storage: Arc<K>,
    pub smt: PhantomData<SMT>,
}

impl <K: KvConnect, SMT: SmtInterface<K::Client>> TreeRepository for TreeRepositoryImpl<K, SMT>
{
    fn update(&self, tree_id: String, root: Option<String>, leaves: Vec<String>) -> Result<String, TreeError> {
        let mut storage: <K as KvConnect>::Client = self.storage.open(Some(tree_id))?;
        Ok(SMT::add_leaves(storage, root, leaves)?)
    }

    fn proof(&self, tree_id: String, root: String, leaves: Vec<String>) -> Result<Proof, TreeError> {
        let mut storage = self.storage.open_read_only(Some(tree_id))?;
        Ok(SMT::get_proof(storage, root, leaves)?)
    }
}