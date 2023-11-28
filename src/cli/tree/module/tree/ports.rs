use serde::{Deserialize, Serialize};

use super::{error::TreeError, tree::Proof};



#[derive(Serialize, Deserialize)]
pub struct UpdateTreeInput {
    pub leaves: Vec<String>,
    pub shard_id: Option<String>,
    pub state_id: Option<i32>,
    pub root: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTreeOutput {
    pub root: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetProofInput {
    pub leaves: Vec<String>,
    pub shard_id: Option<String>,
    pub state_id: Option<i32>,
    pub root: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetProofOutput {
    pub nodes: Vec<String>,
    pub bitmap: String,
    pub depth: String,
}

#[derive(Serialize, Deserialize)]
pub struct HealthOutput {
    pub success: bool,
}

pub trait TreeRepository {
    fn update(
        &self,
        tree_id: String,
        root: Option<String>,
        leaves: Vec<String>,
    ) -> Result<String, TreeError>;
    fn proof(&self, tree_id: String, root: String, leaves: Vec<String>)
        -> Result<Proof, TreeError>;
}

pub trait TreeService {
    fn update(&self, input: UpdateTreeInput) -> Result<UpdateTreeOutput, TreeError>;
    fn proof(&self, input: GetProofInput) -> Result<GetProofOutput, TreeError>;
}
