library;

pub enum TokenError {
    DoesNotExist: (),
    NotOwner: (),
    NotAuthorized: (),
}