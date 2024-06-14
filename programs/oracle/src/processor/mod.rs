use borsh::BorshDeserialize;
use common::VariantName;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

mod assertion;
mod request;
mod voting;

mod create_currency_v1;
mod create_oracle_v1;
mod update_oracle_v1;

pub(crate) use self::create_currency_v1::*;
pub(crate) use self::create_oracle_v1::*;
pub(crate) use self::update_oracle_v1::*;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    use crate::instruction::OracleInstruction as I;

    let instruction = I::try_from_slice(instruction_data)?;

    log!("Instruction: {}", instruction.variant_name());

    match instruction {
        I::CreateOracleV1(args) => create_oracle_v1(program_id, accounts, args),
        I::UpdateOracleV1(args) => update_oracle_v1(program_id, accounts, args),
        I::CreateCurrencyV1(args) => create_currency_v1(program_id, accounts, args),
        I::CreateRequest(args) => request::create(program_id, accounts, args),
        I::CreateAssertion(args) => assertion::create(program_id, accounts, args),
        I::ExpireAssertion(args) => assertion::expire(program_id, accounts, args),
        I::DisputeAssertion(args) => assertion::dispute(program_id, accounts, args),
        I::SubmitVote(args) => voting::submit(program_id, accounts, args),
        I::FinalizeVoting(args) => voting::finalize(program_id, accounts, args),
    }
}
