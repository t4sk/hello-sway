library;

pub enum InitError {
    CannotReinitialize: (),
    ZeroOwners: (),
    MaxOwners: (),
}