use crate::cli::tree::module::{tree::tree::Proof, utils::{self, error::UtilsError}};
use std::convert::TryFrom;

use bloock_smt::{proof::Proof as SmtProof, tree::SparseMerkleTree};

use super::ports::{SmtError, SmtInterface};



cfg_if::cfg_if! {
    if #[cfg(feature = "blake2b")] {
        pub use bloock_merge::hash_algorithms::blake2b::Blake2b as HashAlgorithm;
    } else if #[cfg(feature = "keccak")] {
        pub use bloock_merge::hash_algorithms::keccak::Keccak as HashAlgorithm;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "h256")] {
        pub use bloock_types::bytes::h256;
        pub use bloock_types::bytes::h256::H256 as HashType;
    } else if #[cfg(feature = "h128")] {
        pub use bloock_types::bytes::h128;
        pub use bloock_types::bytes::h128::H128 as HashType;
    }
}

pub struct SmtImpl {}

impl<D: bloock_storage::kv::KeyValue + 'static> SmtInterface<D> for SmtImpl {
    fn add_leaves(
        mut storage: D,
        root: Option<String>,
        messages: Vec<String>,
    ) -> Result<String, SmtError> {
        let leaves_bytes: Vec<HashType> = utils::hex::iter_decode_hex(messages)?;

        let root_bytes = match &root {
            Some(hash) => Some(utils::hex::decode_hex::<HashType>(hash)).transpose(),
            None => Ok(None),
        }?;

        let smt = match root_bytes {
            Some(r) => {
                let mut smt =
                    SparseMerkleTree::<HashType, D, HashAlgorithm>::load(r, &mut storage)?;
                smt.add_leaves(leaves_bytes)?;
                smt
            }
            None => SparseMerkleTree::new(&mut storage, Some(leaves_bytes))?,
        };

        match smt.get_root() {
            Some(r) => Ok(utils::hex::encode_hex(r)),
            None => Ok("".to_string()),
        }
    }

    fn get_proof(mut storage: D, root: String, messages: Vec<String>) -> Result<Proof, SmtError> {
        let leaves_bytes: Vec<HashType> = utils::hex::iter_decode_hex(messages)?;
        let root_bytes = utils::hex::decode_hex::<HashType>(&root)?;

        let smt = SparseMerkleTree::<HashType, _, HashAlgorithm>::load(root_bytes, &mut storage)?;

        let proof = smt.get_multiproof(leaves_bytes)?;

        Ok(Proof::try_from(proof)?)
    }
}

impl TryFrom<SmtProof<HashType, HashAlgorithm>> for Proof {
    type Error = SmtError;
    fn try_from(proof: SmtProof<HashType, HashAlgorithm>) -> Result<Self, Self::Error> {
        let nodes: Vec<String> = proof
            .hashes
            .iter()
            .map(|hash| utils::hex::encode_hex(hash))
            .collect();

        let mut bitmap = utils::hex::encode_hex(proof.bitmap.as_slice());

        let depth_u16: Vec<u16> = proof
            .depths
            .iter()
            .map(|&p| u16::try_from(p).map_err(|_| UtilsError::HexError("hello".to_string())))
            .collect::<Result<Vec<u16>, UtilsError>>()?;

        let mut depth_u8: Vec<u8> = vec![];
        for x in depth_u16 {
            depth_u8.append(&mut x.to_be_bytes().to_vec());
        }

        let depth = utils::hex::encode_hex(&depth_u8);

        Ok(Proof {
            nodes,
            bitmap,
            depth,
        })
    }
}
