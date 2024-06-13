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
    pub reward_range: (u64, u64),
    /// The valid bond range when creating an [`Assertion`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    pub bond_range: (u64, u64),
}

impl Account for Currency {
    const TYPE: AccountType = AccountType::Currency;
}

impl From<InitCurrency> for (Currency, usize) {
    fn from(params: InitCurrency) -> (Currency, usize) {
        let InitCurrency { mint, reward_range, bond_range } = params;

        (Currency { account_type: Currency::TYPE, mint, reward_range, bond_range }, Currency::SIZE)
    }
}

pub(crate) struct InitCurrency {
    pub mint: Pubkey,
    pub reward_range: (u64, u64),
    pub bond_range: (u64, u64),
}
