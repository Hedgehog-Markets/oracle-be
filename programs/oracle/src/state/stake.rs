use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::clock::UnixTimestamp;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

// TODO:
// - unstaking
// - merging staking accounts
//   - cannot be done if an account has a lock for an active vote

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Stake {
    account_type: AccountType,

    /// Owner of the stake.
    pub owner: Pubkey,
    /// Address the stake is delegated to.
    ///
    /// The delegate can vote and restake rewards, but cannot withdraw stake.
    pub delegate: Pubkey,

    /// The amount staked.
    pub amount: u64,

    /// The Unix timestamp the stake is locked until.
    pub lock_timestamp: UnixTimestamp,
}

impl Account for Stake {
    const TYPE: AccountType = AccountType::Stake;
}
