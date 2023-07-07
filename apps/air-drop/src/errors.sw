library;

pub enum DepositError {
    ZeroAmount: (),
    NotAirDropAsset: (),
}

pub enum ClaimError {
    AlreadyClaimed: (),
    InvalidMerkleProof: (),
}
