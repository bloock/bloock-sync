use anchor::Anchor;
use serde::Deserialize;
use super::http::{bloock_http::BloockHttpClient, Client};

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
    max_anchor: u64
}

impl SyncService {
    pub fn new(http_client: BloockHttpClient, max_anchor: u64) -> Self {
        Self {
            http_client,
            max_anchor
        }
    }

    pub async fn build_tree(&self) -> String {
        for index in 1616..=self.max_anchor {
            let base_url = self.http_client.get_api_host();
            let url = format!("{base_url}/core/anchor/{index}");
            let anchor: Anchor = match self.http_client.get_json::<String, Anchor>(url, None).await {
                Ok(res) => res,
                Err(e) => return e.to_string(),
            };
            let ipfs_cid: String = match anchor.blocks_cid {
                Some(res) => res,
                None => continue
            };

            
            let url = format!("{}/hosting/v1/ipfs/{}", base_url, ipfs_cid);
            let cids: Vec<String> = match self.http_client.get_json::<String, Vec<String>>(url, None).await {
                Ok(res) => res,
                Err(e) => return e.to_string(),
            };

            
            println!("CID {:?} inserted to the tree", cids)
        }


        "".to_string()
    }

    
}