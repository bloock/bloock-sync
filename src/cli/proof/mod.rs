use std::{marker::PhantomData, sync::Arc};
use super::tree::{config::configure_kv, infrastructure::smt::SmtImpl, module::tree::{ports::TreeRepository, repository::TreeRepositoryImpl, tree::Proof}};

pub struct ProofService {
    root: String,
    tree_id: String
}

impl ProofService {
    pub fn new(root: String, tree_id: String) -> Self {
        Self {
            root,
            tree_id
        }
    }

    pub fn get_proof(&self, hashes: Vec<String>) -> Result<Proof, String> {
        let tree_repo = TreeRepositoryImpl {
            storage: Arc::new(configure_kv()),
            smt: PhantomData::<SmtImpl>,
        };

        match tree_repo.proof(self.tree_id.clone(), self.root.clone(), hashes) {
            Ok(proof) => Ok(proof),
            Err(e) => Err(e.to_string()),
        }
    }
}

