use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, FinalizeVotingAccounts};
use crate::instruction::FinalizeVotingArgs;
use crate::pda;
use crate::state::{Account, AccountSized, OracleV1, RequestState, RequestV1, VotingV1};

pub fn finalize<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: FinalizeVotingArgs,
) -> ProgramResult {
    let ctx = FinalizeVotingAccounts::context(accounts)?;

    match args {
        FinalizeVotingArgs::V1 { .. } => finalize_v1(program_id, ctx, args),
    }
}

fn finalize_v1(
    _program_id: &Pubkey,
    ctx: Context<FinalizeVotingAccounts>,
    args: FinalizeVotingArgs,
) -> ProgramResult {
    let FinalizeVotingArgs::V1 {} = args;

    let FinalizeVotingAccounts { oracle, request, voting } = ctx.accounts;

    let voting_window: u32;

    // Get oracle voting window.
    {
        let oracle = OracleV1::from_account_info(oracle)?;

        voting_window = oracle.config.voting_window;
    }

    let request_address = request.key;

    let mut request = RequestV1::from_account_info_mut(request)?;

    // Check request.
    {
        pda::request::assert_pda(request_address, oracle.key, &request.index)?;

        // Check request is not disputed.
        if request.state != RequestState::Disputed {
            return Err(OracleError::NotDisputed.into());
        }
    }

    pda::voting::assert_pda(voting.key, request_address)?;

    let now = Clock::get()?;

    let mut voting = VotingV1::from_account_info_mut(voting)?;

    // Check the voting window has expired.
    if now.unix_timestamp < voting.end_timestamp {
        return Err(OracleError::VotingWindowNotExpired.into());
    }

    // If no votes were cast then start a new vote window.
    if voting.vote_count == 0 {
        log!("No votes cast - starting new vote window");

        voting.start_timestamp = now.unix_timestamp;
        voting.end_timestamp = checked_add!(now.unix_timestamp, i64::from(voting_window))?;

        voting.save()?;

        // TODO: Emit an event?

        return Ok(());
    }

    // Voting account is not mutated when resolving.
    let voting = voting.into_inner();

    // Update request with resolved value.
    request.resolve_timestamp = now.unix_timestamp;
    request.state = RequestState::Resolved;
    request.value = voting.mode_value;

    request.save()?;

    // TODO: Emit an event?

    Ok(())
}
