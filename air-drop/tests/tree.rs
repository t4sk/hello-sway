use fuel_merkle::{binary::in_memory::MerkleTree, common::Bytes32};
use fuels::types::{Bits256, Identity};
use sha2::{Digest, Sha256};

pub(crate) fn build(
    key: u64,
    leaves: Vec<(Identity, u64)>,
) -> (MerkleTree, Bits256, Bytes32, Vec<Bits256>) {
    let mut tree = MerkleTree::new();

    for data in leaves.iter() {
        let mut hasher = Sha256::new();
        let identity = data.0.clone();

        match identity {
            // Encoding enum type + identity data
            Identity::Address(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 0]);
                hasher.update(*identity);
            }
            Identity::ContractId(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
                hasher.update(*identity);
            }
        }
        // Appending amount
        hasher.update(data.1.to_be_bytes());

        let digest: [u8; 32] = hasher.finalize().try_into().unwrap();
        tree.push(&digest);
    }

    let merkle_root = tree.root();
    // Returns (Bytes32, ProofSet = Vec<Bytes32>)
    let mut proof = tree.prove(key).unwrap();
    // Remove merkle leaf from proof
    let merkle_leaf = proof.1[0];
    proof.1.remove(0);

    let mut final_proof: Vec<Bits256> = Vec::new();
    for node in proof.1 {
        final_proof.push(Bits256(node));
    }

    (tree, Bits256(merkle_root), merkle_leaf, final_proof)
}
