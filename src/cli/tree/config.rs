use bloock_storage::{config::RocksDBConfig, kv::{kv_rocks::RocksDB, KvBuilder}};

pub fn configure_kv() -> KvBuilder<RocksDB> {
    let config = RocksDBConfig {
        base_path: "testsdb/".to_string(),
        create_if_missing: true,
    };
    KvBuilder::<RocksDB> { config }
}