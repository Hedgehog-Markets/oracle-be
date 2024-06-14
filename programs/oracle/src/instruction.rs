use borsh::{BorshDeserialize, BorshSerialize};
use common::VariantName;
use shank::{ShankContext, ShankInstruction};

use crate::processor::*;
use crate::state::RequestData;

#[rustfmt::skip::attributes(account)]
#[derive(Clone, VariantName, ShankContext, ShankInstruction, BorshDeserialize)]
pub enum OracleInstruction {
    /// Creates program oracle.
    #[account(0, writable, name = "oracle", desc = "Oracle account")]
    #[account(1, signer, writable, name = "payer", desc = "Payer")]
    #[account(2, name = "system_program", desc = "System program")]
    CreateOracleV1(CreateOracleV1Args),

    /// Updates program oracle.
    #[account(0, writable, name = "oracle", desc = "Oracle account")]
    #[account(1, signer, name = "authority", desc = "Oracle authority")]
    UpdateOracleV1(UpdateOracleV1Args),

    /// Creates a currency.
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "currency", desc = "Currency")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, name = "authority", desc = "Oracle authority")]
    #[account(4, name = "payer", desc = "Payer")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    #[account(6, name = "system_program", desc = "System program")]
    CreateCurrencyV1(CreateCurrencyV1Args),

    /// Creates a new [`Request`].
    ///
    /// [`Request`]: crate::state::Request
    #[account(0, writable, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "reward_currency", desc = "Reward currency")]
    #[account(3, name = "bond_currency", desc = "Bond currency")]
    #[account(4, name = "reward_mint", desc = "Reward mint")]
    #[account(5, writable, name = "reward_source", desc = "Reward source token account")]
    #[account(6, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(7, signer, name = "creator", desc = "Creator")]
    #[account(8, signer, writable, name = "payer", desc = "Payer")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    CreateRequest(CreateRequestArgs),

    /// Creates an [`Assertion`] for a [`Request`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    /// [`Request`]: crate::state::Request
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "assertion", desc = "Assertion")]
    #[account(3, name = "bond_mint", desc = "Bond mint")]
    #[account(4, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(5, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(6, signer, name = "asserter", desc = "Asserter")]
    #[account(7, signer, writable, name = "payer", desc = "Payer")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    #[account(9, name = "system_program", desc = "System program")]
    CreateAssertion(CreateAssertionArgs),

    /// Resolves an [`Assertion`] after the expiration timestamp.
    ///
    /// [`Assertion`]: crate::state::Assertion
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "assertion", desc = "Assertion")]
    ExpireAssertion(ExpireAssertionArgs),

    /// Disputes an [`Assertion`] for a [`Request`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    /// [`Request`]: crate::state::Request
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "assertion", desc = "Assertion")]
    #[account(3, writable, name = "voting", desc = "Voting")]
    #[account(4, name = "bond_mint", desc = "Bond mint")]
    #[account(5, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(6, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(7, signer, name = "disputer", desc = "Disputer")]
    #[account(8, signer, writable, name = "payer", desc = "Payer")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    DisputeAssertion(DisputeAssertionArgs),

    /// Submits a [`Vote`] for [`Voting`].
    ///
    /// [`Vote`]: crate::state::Vote
    /// [`Voting`]: crate::state::Voting
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    #[account(3, writable, name = "vote", desc = "Vote")]
    #[account(4, name = "stake", desc = "Stake")]
    #[account(5, signer, name = "voter", desc = "Voter")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "system_program", desc = "System program")]
    SubmitVote(SubmitVoteArgs),

    /// Finalizes [`Voting`].
    ///
    /// [`Voting`]: crate::state::Voting
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    FinalizeVoting(FinalizeVotingArgs),
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum CreateRequestArgs {
    V1 {
        /// Amount rewarded to the asserter/disputer on resolution.
        reward: u64,
        /// Amount to required to bond in order to assert/dispute value.
        bond: u64,
        /// Unix timestamp after which a value can be asserted.
        timestamp: i64,
        /// Request data.
        data: RequestData,
    },
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum CreateAssertionArgs {
    V1 {
        /// Value to assert.
        value: u64,
    },
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum ExpireAssertionArgs {
    V1 {},
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum DisputeAssertionArgs {
    V1 {
        /// Value to dispute assertion with.
        value: u64,
    },
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum SubmitVoteArgs {
    V1 {
        /// Value to vote for.
        value: u64,
    },
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum FinalizeVotingArgs {
    V1 {},
}
