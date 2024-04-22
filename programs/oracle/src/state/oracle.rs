use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Oracle {
    account_type: AccountType,

    /// Index for the next request.
    pub next_index: u64,

    /// Authority address.
    pub authority: Pubkey,

    /// Oracle config.
    pub config: Config,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Config {
    /// The duration of the dispute window in seconds.
    pub dispute_window: i64,
    /// The duration of the voting window in seconds.
    pub voting_window: i64,
    /// The fee taken, in basis points, from the bond of the incorrect party in a dispute.
    pub bond_fee_bps: u16,
}

impl Account for Oracle {
    const TYPE: AccountType = AccountType::Oracle;
}

impl From<InitOracle> for (Oracle, usize) {
    fn from(params: InitOracle) -> (Oracle, usize) {
        let InitOracle { authority, config } = params;

        (Oracle { account_type: Oracle::TYPE, next_index: 0, authority, config }, Oracle::SIZE)
    }
}

pub(crate) struct InitOracle {
    pub authority: Pubkey,
    pub config: Config,
}
