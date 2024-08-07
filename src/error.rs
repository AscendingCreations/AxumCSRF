use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsrfError {
    #[error("Token could not be hashed.")]
    PasswordHash,
    #[error("Verification Failed.")]
    Verify,
    #[error("Could not Encode Salt.")]
    Salt,
    #[error("Could not Hash Token.")]
    Token,
}
