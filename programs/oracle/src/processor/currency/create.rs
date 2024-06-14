use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, CreateCurrencyAccounts};
use crate::instruction::CreateCurrencyArgs;
use crate::state::{Account, CurrencyV1, InitAccount, InitContext, InitCurrency, OracleV1};
use crate::{pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateCurrencyArgs,
) -> ProgramResult {
    let ctx = CreateCurrencyAccounts::context(accounts)?;

    match args {
        CreateCurrencyArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateCurrencyAccounts>,
    args: CreateCurrencyArgs,
) -> ProgramResult {
    let CreateCurrencyArgs::V1 { reward_range, bond_range } = args;

    let CreateCurrencyAccounts {
        oracle,
        currency,
        mint,
        authority,
        payer,
        token_program,
        system_program,
    } = ctx.accounts;

    if !authority.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_token_program(token_program.key)?;
    utils::assert_system_program(system_program.key)?;

    pda::oracle::assert_pda(oracle.key)?;

    // Step 1: Check authority.
    {
        let oracle = OracleV1::from_account_info(oracle)?;

        if !common::cmp_pubkeys(&oracle.authority, authority.key) {
            return Err(OracleError::OracleAuthorityMismatch.into());
        }
    }

    // Step 2: Initialize `currency` account.
    {
        let currency_bump =
            pda::currency::assert_pda(currency.key, oracle.key, mint.key, token_program.key)?;
        let signer_seeds =
            pda::currency::seeds_with_bump(oracle.key, mint.key, token_program.key, &currency_bump);

        CurrencyV1::init(InitCurrency { mint: *mint.key, reward_range, bond_range }).save(
            InitContext {
                account: currency,
                payer,
                system_program,
                program_id,
                signers_seeds: &[&signer_seeds],
            },
        )?;
    }

    Ok(())
}
