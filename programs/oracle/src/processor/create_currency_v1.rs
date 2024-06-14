use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateCurrencyV1Accounts;
use crate::state::{Account, CurrencyV1, InitAccount, InitContext, InitCurrency, OracleV1};
use crate::{pda, utils};

// TODO: Validate ranges.

#[derive(Clone, BorshDeserialize)]
pub struct CreateCurrencyV1Args {
    /// The valid reward range when creating a [`Request`].
    ///
    /// [`Request`]: crate::state::Request
    pub reward_range: (u64, u64),
    /// The valid bond range when creating an [`Assertion`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    pub bond_range: (u64, u64),
}

pub fn create_currency_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateCurrencyV1Args,
) -> ProgramResult {
    let ctx = CreateCurrencyV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::oracle::assert_pda(ctx.accounts.oracle.key)?;

    // Step 1: Check oracle authority.
    OracleV1::from_account_info(ctx.accounts.oracle)?
        .assert_authority(ctx.accounts.authority.key)?;

    // Step 2: Initialize currency account.
    {
        let bump = pda::currency::assert_pda(
            ctx.accounts.currency.key,
            ctx.accounts.oracle.key,
            ctx.accounts.mint.key,
        )?;
        let signer_seeds =
            pda::currency::seeds_with_bump(ctx.accounts.oracle.key, ctx.accounts.mint.key, &bump);

        CurrencyV1::init(InitCurrency {
            mint: *ctx.accounts.mint.key,
            reward_range: args.reward_range,
            bond_range: args.bond_range,
        })
        .save(InitContext {
            account: ctx.accounts.currency,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
