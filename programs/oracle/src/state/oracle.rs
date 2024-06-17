use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct OracleV1 {
    account_type: AccountType,

    /// Index for the next request.
    pub next_index: u64,

    /// Authority address.
    pub authority: Pubkey,
    /// Governance token mint address.
    pub governance_mint: Pubkey,

    /// Oracle config.
    pub config: Config,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Config {
    /// The fee taken, in basis points, from the bond of the incorrect party in a dispute.
    pub bond_fee_bps: u16,

    /// The duration of the dispute window in seconds.
    pub dispute_window: u32,
    /// The duration of the voting window in seconds.
    pub voting_window: u32,
}

impl OracleV1 {
    pub fn assert_authority(&self, authority: &Pubkey) -> Result<(), OracleError> {
        if !common::cmp_pubkeys(&self.authority, authority) {
            return Err(OracleError::OracleAuthorityMismatch);
        }
        Ok(())
    }
}

impl Account for OracleV1 {
    const TYPE: AccountType = AccountType::OracleV1;
}

impl From<InitOracle> for (OracleV1, usize) {
    fn from(params: InitOracle) -> (OracleV1, usize) {
        let InitOracle { authority, governance_mint, config } = params;

        (
            OracleV1 {
                account_type: OracleV1::TYPE,
                next_index: 0,
                authority,
                governance_mint,
                config,
            },
            OracleV1::SIZE,
        )
    }
}

pub(crate) struct InitOracle {
    pub authority: Pubkey,
    pub governance_mint: Pubkey,
    pub config: Config,
}
