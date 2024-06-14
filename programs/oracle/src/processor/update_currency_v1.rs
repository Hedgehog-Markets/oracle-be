use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::UpdateCurrencyV1Accounts;
use crate::state::{Account, AccountSized, CurrencyV1, OracleV1};
use crate::utils::Bounds;
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct UpdateCurrencyV1Args {
    pub new_reward_range: Bounds,
    pub new_bond_range: Bounds,
}

pub fn update_currency_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateCurrencyV1Args,
) -> ProgramResult {
    let ctx = UpdateCurrencyV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;

    // Guard PDAs.
    pda::oracle::assert_pda(ctx.accounts.oracle.key)?;

    // Step 1: Check oracle authority.
    OracleV1::from_account_info(ctx.accounts.oracle)?
        .assert_authority(ctx.accounts.authority.key)?;

    // Step 1: Update currency.
    {
        let mut currency = CurrencyV1::from_account_info_mut(ctx.accounts.currency)?;

        // Guard PDA.
        currency.assert_pda(ctx.accounts.currency.key)?;

        currency.reward_range = args.new_reward_range;
        currency.bond_range = args.new_bond_range;

        currency.save()?;
    }

    Ok(())
}
