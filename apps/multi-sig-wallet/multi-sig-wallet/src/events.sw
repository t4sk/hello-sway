library;

pub struct ExecuteEvent {
    tx_hash: b256,
    nonce: u64,
}

pub struct TransferEvent {
    tx_hash: b256,
    nonce: u64,
}
