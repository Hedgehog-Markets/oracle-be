use std::ops::Range;

use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Currency {
    account_type: AccountType,

    /// The mint address.
    pub mint: Pubkey,

    /// The valid reward range when creating a [`Request`].
    ///
    /// [`Request`]: crate::state::Request
    pub reward_range: Range<u64>,
    /// The valid bond range when creating an [`Assertion`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    pub bond_range: Range<u64>,
}

impl Account for Currency {
    const TYPE: AccountType = AccountType::Currency;
}
