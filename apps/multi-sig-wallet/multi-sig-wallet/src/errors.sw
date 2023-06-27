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
    MinSignatures: (),
}

pub enum AccessControlError {
    NotAuthorized: (),
}
