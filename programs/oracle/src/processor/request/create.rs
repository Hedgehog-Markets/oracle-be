use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, CreateRequestAccounts};
use crate::instruction::CreateRequestArgs;
use crate::state::{
    Account, AccountSized, Currency, InitAccount, InitContext, InitRequest, Oracle, Request,
};
use crate::{pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateRequestArgs,
) -> ProgramResult {
    let ctx = CreateRequestAccounts::context(accounts)?;

    match args {
        CreateRequestArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateRequestAccounts>,
    args: CreateRequestArgs,
) -> ProgramResult {
    let CreateRequestArgs::V1 { bond, reward, timestamp, data } = args;

    let CreateRequestAccounts {
        oracle,
        request,
        reward_currency: reward_currency_info,
        bond_currency: bond_currency_info,
        reward_mint,
        reward_source,
        reward_escrow,
        creator,
        payer,
        token_program,
        system_program,
    } = ctx.accounts;

    if !creator.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_token_program(token_program.key)?;
    utils::assert_system_program(system_program.key)?;

    pda::oracle::assert_pda(oracle.key)?;

    let reward_currency = Currency::from_account_info(reward_currency_info)?;
    let bond_currency = Currency::from_account_info(bond_currency_info)?;

    pda::currency::assert_pda(
        reward_currency_info.key,
        oracle.key,
        &reward_currency.mint,
        token_program.key,
    )?;
    pda::currency::assert_pda(
        bond_currency_info.key,
        oracle.key,
        &bond_currency.mint,
        token_program.key,
    )?;

    // Check the reward mint matches the reward currency.
    if !common::cmp_pubkeys(&reward_currency.mint, reward_mint.key) {
        return Err(OracleError::RewardMintMismatch.into());
    }

    // Check the reward and bond bounds.
    if !range_contains(reward_currency.reward_range, reward) {
        return Err(OracleError::RewardBounds.into());
    }
    if !range_contains(bond_currency.bond_range, bond) {
        return Err(OracleError::BondBounds.into());
    }

    let request_index: u64;

    // Step 1: Get and increment next request index.
    {
        let mut oracle = Oracle::from_account_info_mut(oracle)?;

        request_index = oracle.next_index;

        oracle.next_index = checked_add!(oracle.next_index, 1)?;
        oracle.save()?;
    }

    // Step 2: Initialize `request` account.
    {
        let request_bump = pda::request::assert_pda(request.key, oracle.key, &request_index)?;
        let signer_seeds = pda::request::seeds_with_bump(oracle.key, &request_index, &request_bump);

        Request::try_init(InitRequest {
            index: request_index,
            creator: *creator.key,
            reward,
            reward_mint: *reward_mint.key,
            bond,
            bond_mint: bond_currency.mint,
            timestamp,
            data,
        })?
        .save(InitContext {
            account: request,
            payer,
            system_program,
            program_id,
            signer_seeds: &[&signer_seeds],
        })?;
    }

    // Step 3: Transfer reward to escrow.
    if reward > 0 {
        let mint_decimals = cpi::spl::mint_decimals(reward_mint)?;

        // Step 3.1: Initialize `reward_escrow` account.
        {
            let reward_bump = pda::reward::assert_pda(reward_escrow.key, request.key)?;
            let signer_seeds = pda::reward::seeds_with_bump(request.key, &reward_bump);

            cpi::spl::create_token_account(
                request.key,
                cpi::spl::CreateTokenAccount {
                    account: reward_escrow,
                    mint: reward_mint,
                    payer,
                    token_program,
                    system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 3.2: Transfer reward from `reward_source` to `reward_escrow`.
        cpi::spl::transfer_checked(
            reward,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: reward_source,
                destination: reward_escrow,
                mint: reward_mint,
                authority: creator,
                token_program,
            },
            &[],
        )?;
    }

    // TODO: Emit an event?

    Ok(())
}

const fn range_contains((start, end): (u64, u64), value: u64) -> bool {
    start <= value && value < end
}
