library;

pub struct InitEvent {
    asset: ContractId,
    merkle_root: b256,
    num_leaves: u64,
}

pub struct ClaimEvent {
    index: u64,
    amount: u64,
    sender: Identity,
}