#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Address(#[from] lettre::address::AddressError),
    #[error("Missing name for mailbox")]
    MissingName,
}
