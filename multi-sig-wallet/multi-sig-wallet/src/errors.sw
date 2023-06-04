library;

pub enum InitError {
    CannotReinitialize: (),
    ZeroOwners: (),
    MaxOwners: (),
    DuplicateOwner: (),
}

pub enum SignatureError {
    NotOwner: (),
    IncorrectSignerOrdering: (),
}

pub enum ExecuteError {
    MinSignatures: (),
}
