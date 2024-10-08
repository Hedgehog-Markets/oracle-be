use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize, ShankAccount)]
pub struct AssertionV1 {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// Unix timestamp of the assertion.
    pub assertion_timestamp: i64,
    /// Unix timestamp at which the dispute window expires and the assertion
    /// can be resolved.
    ///
    /// This should be [`assertion_timestamp`] + [`DISPUTE_WINDOW`].
    ///
    /// [`assertion_timestamp`]: Assertion::assertion_timestamp
    /// [`DISPUTE_WINDOW`]: crate::DISPUTE_WINDOW
    pub expiration_timestamp: i64,

    /// Asserter address.
    pub asserter: Pubkey,
    /// Disputer address.
    pub disputer: Pubkey,

    /// Value submitted by the asserter.
    pub asserted_value: u64,
}

impl AssertionV1 {
    pub fn in_dispute_window(&self, timestamp: i64) -> bool {
        timestamp < self.expiration_timestamp
    }

    pub fn assert_asserter(&self, asserter: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(asserter, &self.asserter) {
            return Err(OracleError::AsserterMismatch);
        }
        Ok(())
    }

    pub fn assert_disputer(&self, disputer: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(disputer, &self.disputer) {
            return Err(OracleError::DisputerMismatch);
        }
        Ok(())
    }

    pub fn validate_expiration_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if self.in_dispute_window(timestamp) {
            return Err(OracleError::DisputeWindowNotExpired);
        }
        Ok(())
    }

    pub fn validate_dispute_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if !self.in_dispute_window(timestamp) {
            return Err(OracleError::DisputeWindowExpired);
        }
        Ok(())
    }
}

impl Account for AssertionV1 {
    const TYPE: AccountType = AccountType::AssertionV1;
}

impl TryFrom<InitAssertion> for (AssertionV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitAssertion) -> Result<(AssertionV1, usize), Self::Error> {
        let InitAssertion {
            request,
            assertion_timestamp,
            asserter,
            asserted_value,
            dispute_window,
        } = params;

        let expiration_timestamp = checked_add!(assertion_timestamp, i64::from(dispute_window))?;

        Ok((
            AssertionV1 {
                account_type: AssertionV1::TYPE,
                request,
                assertion_timestamp,
                expiration_timestamp,
                asserter,
                disputer: Pubkey::default(),
                asserted_value,
            },
            AssertionV1::FIXED_SIZE,
        ))
    }
}

pub(crate) struct InitAssertion {
    pub request: Pubkey,

    pub assertion_timestamp: i64,
    pub asserter: Pubkey,
    pub asserted_value: u64,

    pub dispute_window: u32,
}
