use anchor::Anchor;
use bloock_smt::tree::SparseMerkleTree;
use bloock_types::bytes::h256::H256;
use bloock_merge::hash_algorithms::blake2b::Blake2b;
use serde::Deserialize;
use super::http::{bloock_http::BloockHttpClient, Client};
use bloock_storage::kv::kv_hashmap::HashMap as Database;

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
        let mut db = Database::new();
        let smt: SparseMerkleTree<H256, Database<>, Blake2b> =  match SparseMerkleTree::<H256, Database<>, Blake2b>::new(&mut db, None) {
            Ok(res) => res,
            Err(e) => return e.to_string(),
        };

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

            let bytes_vec: Vec<[u8; 32]> = cids
            .iter()
            .map(|hex_string| hex_string_to_bytes(hex_string))
            .collect();

            smt.add_leaves(bytes_vec).map_err(|e| {
                return e.to_string()
            });
            
             
            println!("CID {:?} inserted to the tree", cids)
        }


        "".to_string()
    }

    
}

fn hex_string_to_bytes(hex_string: &str) -> [u8; 32] {
    let mut result = [0u8; 32];
    let bytes = hex::decode(hex_string).expect("Invalid hexadecimal string");

    // Ensure bytes has exactly 32 bytes
    assert_eq!(bytes.len(), 32);

    // Copy bytes into result array
    result.copy_from_slice(&bytes);
    result
}