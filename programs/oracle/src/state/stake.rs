use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

// TODO:
// - unstaking
// - merging staking accounts
//   - cannot be done if an account has a lock for an active vote

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize, ShankAccount)]
pub struct StakeV1 {
    account_type: AccountType,

    /// The mint address.
    pub mint: Pubkey,
    /// The owner address.
    pub owner: Pubkey,
    /// The address the stake is delegated to.
    ///
    /// The delegate can vote and restake rewards, but cannot withdraw stake.
    pub delegate: Pubkey,

    /// The amount staked.
    pub amount: u64,

    /// The Unix timestamp the stake is locked until.
    pub lock_timestamp: i64,
}

impl StakeV1 {
    pub fn assert_voter(&self, voter: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.owner, voter)
            && !solana_utils::pubkeys_eq(&self.delegate, voter)
        {
            return Err(OracleError::StakeVoterMismatch);
        }
        Ok(())
    }
}

impl Account for StakeV1 {
    const TYPE: AccountType = AccountType::StakeV1;
}

impl From<InitStake> for (StakeV1, usize) {
    fn from(params: InitStake) -> (StakeV1, usize) {
        let InitStake { mint, owner, amount } = params;

        (
            StakeV1 {
                account_type: StakeV1::TYPE,
                mint,
                owner,
                delegate: owner,
                amount,
                lock_timestamp: i64::MIN,
            },
            StakeV1::FIXED_SIZE,
        )
    }
}

pub(crate) struct InitStake {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
