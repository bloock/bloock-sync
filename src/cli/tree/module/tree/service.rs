use crate::cli::tree::module::utils;

use super::{error::TreeError, ports::{GetProofInput, GetProofOutput, TreeRepository, TreeService, UpdateTreeInput, UpdateTreeOutput}};

pub struct TreeServiceImpl<
    T: TreeRepository
> {
    pub tree_repository: T
}

impl <T> TreeService for TreeServiceImpl<T>
    where
        T: TreeRepository
{
    fn update(&self, input: UpdateTreeInput) -> Result<UpdateTreeOutput, TreeError> {
        let path = utils::path::calculate_path(input.shard_id, input.state_id)?;
        let root = self.tree_repository.update(path, input.root, input.leaves)?;
        Ok(
            UpdateTreeOutput {
                root,
            }
        )
    }

    fn proof(&self, input: GetProofInput) -> Result<GetProofOutput, TreeError> {
        let path = utils::path::calculate_path(input.shard_id, input.state_id)?;
        let proof = self.tree_repository.proof(path, input.root, input.leaves)?;
        Ok(
            GetProofOutput {
                nodes: proof.nodes,
                bitmap: proof.bitmap,
                depth: proof.depth
            }
        )
    }

}
