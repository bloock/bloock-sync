pub mod ports;
pub mod repository;
pub mod error;
pub mod service;
pub mod tree;

use std::sync::Arc;

use std::marker::PhantomData;

use bloock_storage::kv::KvConnect;
use ports::TreeRepository;

use crate::cli::tree::infrastructure::ports::SmtInterface;

pub fn configure_repository<K: KvConnect + 'static, SMT: SmtInterface<K::Client> + 'static>(
    kv: Arc<K>,
    smt: PhantomData<SMT>,
) -> impl TreeRepository
where {
    repository::TreeRepositoryImpl {
        storage: Arc::clone(&kv),
        smt,
    }
}
