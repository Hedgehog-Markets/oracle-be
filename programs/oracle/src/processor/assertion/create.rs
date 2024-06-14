use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, CreateAssertionAccounts};
use crate::instruction::CreateAssertionArgs;
use crate::state::{
    Account, AccountSized, AssertionV1, InitAccount, InitAssertion, InitContext, OracleV1, RequestV1,
    RequestState,
};
use crate::{pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateAssertionArgs,
) -> ProgramResult {
    let ctx = CreateAssertionAccounts::context(accounts)?;

    match args {
        CreateAssertionArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateAssertionAccounts>,
    args: CreateAssertionArgs,
) -> ProgramResult {
    let CreateAssertionArgs::V1 { value } = args;

    let CreateAssertionAccounts {
        oracle,
        request,
        assertion,
        bond_mint,
        bond_source,
        bond_escrow,
        asserter,
        payer,
        token_program,
        system_program,
    } = ctx.accounts;

    if !asserter.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_token_program(token_program.key)?;
    utils::assert_system_program(system_program.key)?;

    let bond: u64;
    let dispute_window: u32;

    // Get oracle dispute window.
    {
        let oracle = OracleV1::from_account_info(oracle)?;

        dispute_window = oracle.config.dispute_window;
    }

    let now = Clock::get()?;

    // Step 1: Update request state.
    {
        let request_address = request.key;

        let mut request = RequestV1::from_account_info_mut(request)?;

        pda::request::assert_pda(request_address, oracle.key, &request.index)?;

        if request.state != RequestState::Requested {
            return Err(OracleError::AlreadyAsserted.into());
        }

        request.validate_bond_mint(bond_mint.key)?;
        request.validate_assertion_timestamp(now.unix_timestamp)?;

        request.data.validate_value(value)?;

        bond = request.bond;

        request.state = RequestState::Asserted;
        request.save()?;
    }

    // Step 2: Initialize `assertion` account.
    {
        let assertion_bump = pda::assertion::assert_pda(assertion.key, request.key)?;
        let signer_seeds = pda::assertion::seeds_with_bump(request.key, &assertion_bump);

        AssertionV1::try_init(InitAssertion {
            request: *request.key,
            assertion_timestamp: now.unix_timestamp,
            asserter: *asserter.key,
            asserted_value: value,
            dispute_window,
        })?
        .save(InitContext {
            account: assertion,
            payer,
            system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    // Step 3: Transfer bond to escrow.
    {
        let mint_decimals = cpi::spl::mint_decimals(bond_mint)?;

        // Step 3.1: Initialize `bond_escrow` account.
        {
            let bond_bump = pda::assert_bond::assert_pda(bond_escrow.key, request.key)?;
            let signer_seeds = pda::assert_bond::seeds_with_bump(request.key, &bond_bump);

            cpi::spl::create_token_account(
                request.key,
                cpi::spl::CreateTokenAccount {
                    account: bond_escrow,
                    mint: bond_mint,
                    payer,
                    token_program,
                    system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 3.2: Transfer bond from `bond_source` to `bond_escrow`.
        cpi::spl::transfer_checked(
            bond,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: bond_source,
                destination: bond_escrow,
                mint: bond_mint,
                authority: asserter,
                token_program,
            },
            &[],
        )?;
    }

    // TODO: Emit an event?

    Ok(())
}
