library;

pub struct Init {
    asset: ContractId,
    merkle_root: b256,
    num_leaves: u64,
}

pub struct Claim {
    index: u64,
    amount: u64,
    sender: Identity,
}
