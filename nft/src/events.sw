library;

pub struct TransferEvent {
    token_id: u64,
    from: Option<Identity>,
    to: Option<Identity>,
}

pub struct ApprovalEvent {
    owner: Identity,
    spender: Identity,
    token_id: u64,
}

pub struct ApprovalForAllEvent {
    owner: Identity,
    operator: Identity,
    approved: bool,
}
