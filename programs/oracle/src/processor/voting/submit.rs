use std::collections::btree_map::Entry;

use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, SubmitVoteAccounts};
use crate::instruction::SubmitVoteArgs;
use crate::state::{
    Account, AccountSized, InitAccount, InitContext, InitVote, OracleV1, RequestState, RequestV1,
    VoteV1, VotingV1,
};
use crate::{pda, utils};

pub fn submit<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: SubmitVoteArgs,
) -> ProgramResult {
    let ctx = SubmitVoteAccounts::context(accounts)?;

    match args {
        SubmitVoteArgs::V1 { .. } => submit_v1(program_id, ctx, args),
    }
}

fn submit_v1(
    program_id: &Pubkey,
    ctx: Context<SubmitVoteAccounts>,
    args: SubmitVoteArgs,
) -> ProgramResult {
    let SubmitVoteArgs::V1 { value } = args;

    let SubmitVoteAccounts { oracle, request, voting, vote, stake, voter, payer, system_program } =
        ctx.accounts;

    if !voter.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_system_program(system_program.key)?;

    let voting_window: u32;

    // Get oracle voting window.
    {
        let oracle = OracleV1::from_account_info(oracle)?;

        voting_window = oracle.config.voting_window;
    }

    // Check request.
    {
        let request_address = request.key;

        let request = RequestV1::from_account_info(request)?;

        pda::request::assert_pda(request_address, &request.index)?;

        if request.state != RequestState::Disputed {
            return Err(OracleError::NotDisputed.into());
        }
    }

    pda::voting::assert_pda(voting.key, request.key)?;

    let now = Clock::get()?;

    let voting_address = voting.key;

    let mut voting = VotingV1::from_account_info_mut(voting)?;

    // Step 1: Check the voting window hasn't expired.
    if voting.end_timestamp <= now.unix_timestamp {
        if voting.vote_count != 0 {
            return Err(OracleError::VotingWindowExpired.into());
        }

        // If no votes were cast then start a new vote window.
        log!("No votes cast - starting new vote window");

        voting.start_timestamp = now.unix_timestamp;
        voting.end_timestamp = checked_add!(now.unix_timestamp, i64::from(voting_window))?;
    }

    // TODO: Implement staking for votes.
    let votes = 1;

    // Step 2: Initialize `vote` account.
    {
        let vote_bump = pda::vote::assert_pda(vote.key, voting_address, stake.key)?;
        let signer_seeds = pda::vote::seeds_with_bump(voting_address, stake.key, &vote_bump);

        VoteV1::init(InitVote { stake: *stake.key, value, votes }).save(InitContext {
            account: vote,
            payer,
            system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    let freq = match voting.votes.entry(value) {
        Entry::Occupied(mut entry) => {
            let entry = entry.get_mut();
            let freq = checked_add!(entry, votes)?;

            *entry = freq;

            freq
        }
        Entry::Vacant(entry) => {
            entry.insert(votes);

            votes
        }
    };

    voting.vote_count = checked_add!(voting.vote_count, votes)?;

    // Maybe update the mode value.
    if freq > voting.votes.get(&voting.mode_value).copied().unwrap_or_default() {
        voting.mode_value = value;
    }

    voting.realloc(payer, system_program)?;
    voting.save()?;

    // TODO: Emit an event?

    Ok(())
}
