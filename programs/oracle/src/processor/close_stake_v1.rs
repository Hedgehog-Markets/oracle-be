use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::CloseStakeV1Accounts;
use crate::state::{Account, StakeV1};
use crate::{pda, utils};

pub fn close_stake_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = CloseStakeV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;

    let amount: u64;

    // Step 1: Check stake account and get staked amount.
    {
        let stake = StakeV1::from_account_info(ctx.accounts.stake)?;

        // Guard stake.
        stake.assert_owner(ctx.accounts.wallet.key)?;
        stake.assert_mint(ctx.accounts.mint.key)?;

        let now = Clock::get()?.unix_timestamp;

        // The stake cannot be closed if it is currently locked.
        if now < stake.lock_timestamp {
            return Err(OracleError::StakeLocked.into());
        }

        amount = stake.amount;
    }

    // Step 2: Withdraw staked amount from pool.
    {
        // Guard PDAs.
        pda::stake_pool::assert_pda(ctx.accounts.stake_pool.key, ctx.accounts.mint.key)?;

        let bump = pda::oracle::assert_pda(ctx.accounts.oracle.key)?;
        let signer_seeds = pda::oracle::seeds_with_bump(&bump);

        let decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;

        cpi::spl::transfer_checked(
            amount,
            decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.stake_pool,
                destination: ctx.accounts.stake_withdraw,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.oracle,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    // Step 3: Close stake account.
    solana_utils::close_account(ctx.accounts.stake, ctx.accounts.payer)?;

    Ok(())
}
