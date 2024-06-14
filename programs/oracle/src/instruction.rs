use borsh::{BorshDeserialize, BorshSerialize};
use common::VariantName;
use shank::{ShankContext, ShankInstruction};

use crate::processor::*;

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

    /// Updates a currency.
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "currency", desc = "Currency")]
    #[account(2, name = "authority", desc = "Oracle authority")]
    UpdateCurrencyV1(UpdateCurrencyV1Args),

    /// Creates a new request.
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
    CreateRequestV1(CreateRequestV1Args),

    /// Creates an assertion for a request.
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
    CreateAssertionV1(CreateAssertionV1Args),

    /// Resolves an undisputed assertion after the expiration timestamp.
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "assertion", desc = "Assertion")]
    ResolveAssertionV1,

    /// Disputes the assertion for a request.
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
    DisputeAssertionV1,

    /// Submits a vote for resolving a disputed assertion.
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    #[account(3, writable, name = "vote", desc = "Vote")]
    #[account(4, name = "stake", desc = "Stake")]
    #[account(5, signer, name = "voter", desc = "Voter")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "system_program", desc = "System program")]
    SubmitVoteV1(SubmitVoteV1Args),

    /// Finalizes [`Voting`].
    ///
    /// [`Voting`]: crate::state::Voting
    #[account(0, name = "oracle", desc = "Oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    FinalizeVoting(FinalizeVotingArgs),
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum FinalizeVotingArgs {
    V1 {},
}
