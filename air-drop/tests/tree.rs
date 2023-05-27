use fuel_merkle::{
    binary::in_memory::MerkleTree,
    common::{empty_sum_sha256, Bytes32, LEAF, NODE},
};
use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Config, Contract, ContractId,
         StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    tx::AssetId,
    types::{Bits256, Identity},
};
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct Node {
    hash: Bytes32,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(hash: Bytes32) -> Self {
        Node {
            hash,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: usize) -> Self {
        self.left = Some(node);
        self
    }

    fn right(mut self, node: usize) -> Self {
        self.right = Some(node);
        self
    }
}

pub(crate) async fn build(
    key: u64,
    leaves: Vec<(Identity, u64)>,
) -> (MerkleTree, Bits256, Bytes32, Vec<Bits256>) {
    let mut tree = MerkleTree::new();

    for data in leaves.iter() {
        let mut hasher = Sha256::new();
        let identity = data.0.clone();

        match identity {
            Identity::Address(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 0]);
                hasher.update(*identity);
            }
            // TODO: what's going on here?
            Identity::ContractId(identity) => {
                hasher.update([0, 0, 0, 0, 0, 0, 0, 1]);
                hasher.update(*identity);
            }
        }
        hasher.update(data.1.to_be_bytes());

        let digest: [u8; 32] = hasher.finalize().try_into().unwrap();
        tree.push(&digest);
    }

    let merkle_root = tree.root();
    let mut proof = tree.prove(key).unwrap();
    // TODO: what?
    let merkle_leaf = proof.1[0];
    proof.1.remove(0);

    let mut final_proof: Vec<Bits256> = Vec::new();

    for iterator in proof.1 {
        // TODO: what?
        final_proof.push(Bits256(iterator));
    }

    (tree, Bits256(merkle_root), merkle_leaf, final_proof)
}