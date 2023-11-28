use bloock_storage::kv::{KvBuilder, KvConnect};


cfg_if::cfg_if! {
    if #[cfg(all(feature = "rocksdb"))] {
        pub use bloock_storage::kv::kv_rocks::RocksDB;
        pub use bloock_storage::config::RocksDBConfig;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "default", not(test)))] {
        pub fn configure_storage(config: RocksDBConfig) -> impl KvConnect { KvBuilder::<RocksDB> { config } }
    }
}
