use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum OracleError {
    /// 0 - Insufficient bond.
    #[error("Insufficient bond")]
    InsufficientBond,

    /// 1 - Not asserted.
    #[error("Request does not have an assertion")]
    NotAsserted,

    /// 2 - Not disputed.
    #[error("Request is not disputed")]
    NotDisputed,

    /// 3 - Already asserted.
    #[error("Request already has an assertion")]
    AlreadyAsserted,

    /// 4 - Already disputed.
    #[error("Assertion has already been disputed")]
    AlreadyDisputed,

    /// 5 - Already resolved.
    #[error("Request has already been resolved")]
    AlreadyResolved,

    /// 6 - Request not accepting assertions yet.
    #[error("Request is not accepting assertion yet")]
    AssertionTooEarly,

    /// 7 - Dispute window has not expired.
    #[error("Dispute window has not expired")]
    DisputeWindowNotExpired,

    /// 8 - Dispute window has expired.
    #[error("Dispute window has expired")]
    DisputeWindowExpired,

    /// 9 - Invalid value.
    #[error("Value is not valid for the request")]
    InvalidValue,

    /// 10 - Invalid dispute.
    #[error("Disputed value falls within range of acceptable deviation for asserted value")]
    InvalidDispute,

    /// 11 - Invalid disputer.
    #[error("Disputer cannot be the same as the asserter")]
    DisputerIsAsserter,

    /// 12 - Bond mint mismatch.
    #[error("Bond mint address does not match")]
    BondMintMismatch,

    /// 13 - Voting window has not expired.
    #[error("Voting window has not expired")]
    VotingWindowNotExpired,

    /// 14 - Voting window has expired.
    #[error("Voting window has expired")]
    VotingWindowExpired,

    /// 15 - Currency mint mismatch.
    #[error("Currency mint address does not match")]
    CurrencyMintMismatch,

    /// 16 - Reward out of bounds.
    #[error("Reward must be within valid bounds")]
    RewardBounds,

    /// 17 - Bond out of bounds.
    #[error("Bond must be within valid bounds")]
    BondBounds,

    /// 19 - Oracle authority mismatch.
    #[error("Oracle authority address does not match")]
    OracleAuthorityMismatch,

    /// 20 - Deserialization error.
    #[error("Failed to deserialize account")]
    DeserializationError,

    /// 21 - Serialization error.
    #[error("Failed to serialize account")]
    SerializationError,

    /// 22 - Stake voter mismatch.
    #[error("Stake delegate does not match voter")]
    StakeVoterMismatch,

    /// 23 - Stake mint mismatch.
    #[error("Stake mint address does not match")]
    StakeMintMismatch,

    /// 24 - Arbitration window has not expired.
    #[error("Arbitration window has not expired")]
    ArbitrationWindowNotExpired,
}

impl PrintProgramError for OracleError {
    fn print<E>(&self) {
        log!("Error: {self}");
    }
}

impl From<OracleError> for ProgramError {
    fn from(e: OracleError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for OracleError {
    fn type_of() -> &'static str {
        "OracleError"
    }
}
