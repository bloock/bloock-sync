use std::{marker::PhantomData, sync::Arc};

use anchor::Anchor;
use indicatif::ProgressBar;
use serde::Deserialize;
use super::{http::{bloock_http::BloockHttpClient, Client}, tree::{config::configure_kv, infrastructure::smt::SmtImpl, module::tree::{ports::TreeRepository, repository::TreeRepositoryImpl}}};
use bloock_storage::{config::RocksDBConfig, kv::{kv_rocks::RocksDB, KvBuilder}};

mod anchor;

#[derive(Deserialize)]
pub struct GetListAnchorsResponse {
    pub meta: PaginationResponse,
}

#[derive(Deserialize)]
pub struct PaginationResponse {
    pub total: u64,
}

pub struct SyncService {
    http_client: BloockHttpClient,
    max_anchor: u64,
    tree_id: String,
}

impl SyncService {
    pub fn new(http_client: BloockHttpClient, max_anchor: u64, tree_id: String) -> Self {
        Self {
            http_client,
            max_anchor,
            tree_id,
        }
    }

    pub async fn build_tree(&self) -> String {
        let pb = indicatif::ProgressBar::new(self.max_anchor.clone());

        let tree_repo = TreeRepositoryImpl {
            storage: Arc::new(configure_kv()),
            smt: PhantomData::<SmtImpl>,
        };

        let mut root = None;
   

        for index in 1..=self.max_anchor {
            let base_url = self.http_client.get_api_host();
            let url = format!("{base_url}/core/anchor/{index}");
            let anchor: Anchor = match self.http_client.get_json::<String, Anchor>(url, None).await {
                Ok(res) => res,
                Err(e) => return e.to_string(),
            };

            pb.println(format!("[+] fetched #{}", anchor.id));
            pb.inc(1);

            let ipfs_cid: String = match anchor.blocks_cid {
                Some(res) => res,
                None => continue
            };
            
            let url = format!("{}/hosting/v1/ipfs/{}", base_url, ipfs_cid);
            let cids: Vec<String> = match self.http_client.get_json::<String, Vec<String>>(url, None).await {
                Ok(res) => res,
                Err(e) => return e.to_string(),
            };

            let smt_root = match tree_repo.update(self.tree_id.clone(), root.clone(), cids) {
                Ok(root) => root,
                Err(e) => return e.to_string(),
            };
            root = Some(smt_root.clone());
        }

        pb.finish_and_clear();

        match root {
            Some(r) => return r,
            None => "".to_string(),
        }
    }    

    pub fn delete_tree(&self) {
        let _ = std::fs::remove_dir_all("testsdb").ok();
    }
}