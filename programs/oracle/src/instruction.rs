use borsh::{BorshDeserialize, BorshSerialize};
use common::{BuildInstruction, VariantName};
use shank::{ShankContext, ShankInstruction};
use solana_program::pubkey::Pubkey;

use crate::state::RequestData;

#[rustfmt::skip::attributes(account)]
#[derive(
    Clone,
    VariantName,
    BuildInstruction,
    ShankContext,
    ShankInstruction,
    BorshDeserialize,
    BorshSerialize,
)]
pub enum OracleInstruction {
    /// Creates program [`Oracle`].
    ///
    /// [`Oracle`]: crate::state::Oracle
    #[account(0, writable, name = "oracle", desc = "Program oracle account")]
    #[account(1, signer, writable, name = "payer", desc = "Payer")]
    #[account(2, name = "system_program", desc = "System program")]
    CreateOracle(CreateOracleArgs),

    /// Creates a new [`Request`].
    ///
    /// [`Request`]: crate::state::Request
    #[account(0, writable, name = "oracle", desc = "Program oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "reward_mint", desc = "Reward mint")]
    #[account(3, writable, name = "reward_source", desc = "Reward source token account")]
    #[account(4, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(5, name = "bond_mint", desc = "Bond mint")]
    #[account(6, signer, name = "creator", desc = "Creator")]
    #[account(7, signer, writable, name = "payer", desc = "Payer")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    #[account(9, name = "system_program", desc = "System program")]
    CreateRequest(CreateRequestArgs),

    /// Creates an [`Assertion`] for a [`Request`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    /// [`Request`]: crate::state::Request
    #[account(0, writable, name = "request", desc = "Request")]
    #[account(1, writable, name = "assertion", desc = "Assertion")]
    #[account(2, name = "bond_mint", desc = "Bond mint")]
    #[account(3, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(4, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(5, signer, name = "asserter", desc = "Asserter")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "system_program", desc = "System program")]
    CreateAssertion(CreateAssertionArgs),

    /// Resolves an [`Assertion`] after the expiration timestamp.
    ///
    /// [`Assertion`]: crate::state::Assertion
    #[account(0, writable, name = "request", desc = "Request")]
    #[account(1, name = "assertion", desc = "Assertion")]
    ExpireAssertion(ExpireAssertionArgs),

    /// Disputes an [`Assertion`] for a [`Request`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    /// [`Request`]: crate::state::Request
    #[account(0, writable, name = "request", desc = "Request")]
    #[account(1, writable, name = "assertion", desc = "Assertion")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    #[account(3, name = "bond_mint", desc = "Bond mint")]
    #[account(4, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(5, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(6, signer, name = "disputer", desc = "Disputer")]
    #[account(7, signer, writable, name = "payer", desc = "Payer")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    #[account(9, name = "system_program", desc = "System program")]
    DisputeAssertion(DisputeAssertionArgs),

    /// Submits a [`Vote`] for [`Voting`].
    ///
    /// [`Vote`]: crate::state::Vote
    /// [`Voting`]: crate::state::Voting
    #[account(0, name = "request", desc = "Request")]
    #[account(1, writable, name = "voting", desc = "Voting")]
    #[account(2, writable, name = "vote", desc = "Vote")]
    #[account(3, name = "stake", desc = "Stake")]
    #[account(4, signer, name = "voter", desc = "Voter")]
    #[account(5, signer, writable, name = "payer", desc = "Payer")]
    #[account(6, name = "system_program", desc = "System program")]
    SubmitVote(SubmitVoteArgs),

    /// Finalizes [`Voting`].
    ///
    /// [`Voting`]: crate::state::Voting
    #[account(0, writable, name = "request", desc = "Request")]
    #[account(1, writable, name = "voting", desc = "Voting")]
    FinalizeVoting(FinalizeVotingArgs),
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateOracleArgs {
    V1 {
        /// Authority.
        authority: Pubkey,
    },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
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

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateAssertionArgs {
    V1 {
        /// Value to assert.
        value: u64,
    },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ExpireAssertionArgs {
    V1 {},
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DisputeAssertionArgs {
    V1 {
        /// Value to dispute assertion with.
        value: u64,
    },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SubmitVoteArgs {
    V1 {
        /// Value to vote for.
        value: u64,
    },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum FinalizeVotingArgs {
    V1 {},
}
