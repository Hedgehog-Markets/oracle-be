use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct CurrencyV1 {
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

impl Account for CurrencyV1 {
    const TYPE: AccountType = AccountType::CurrencyV1;
}

impl From<InitCurrency> for (CurrencyV1, usize) {
    fn from(params: InitCurrency) -> (CurrencyV1, usize) {
        let InitCurrency { mint, reward_range, bond_range } = params;

        (
            CurrencyV1 { account_type: CurrencyV1::TYPE, mint, reward_range, bond_range },
            CurrencyV1::SIZE,
        )
    }
}

pub(crate) struct InitCurrency {
    pub mint: Pubkey,
    pub reward_range: (u64, u64),
    pub bond_range: (u64, u64),
}
