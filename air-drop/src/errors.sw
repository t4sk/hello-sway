library;

pub enum InitError {
    CannotReinitialize: (),
    ZeroTokens: (),
}

pub enum ClaimError {
    NotInitialized: (),
    AlreadyClaimed: (),
    InvalidMerkleProof: (),
    InsufficientBalance: (),
}
