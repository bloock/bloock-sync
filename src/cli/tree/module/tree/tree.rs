use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub nodes: Vec<String>,
    pub bitmap: String,
    pub depth: String,
}