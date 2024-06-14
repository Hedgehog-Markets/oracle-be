use borsh::BorshDeserialize;
use common::VariantName;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

mod assertion;
mod voting;

mod create_assertion_v1;
mod create_currency_v1;
mod create_oracle_v1;
mod create_request_v1;
mod resolve_assertion_v1;
mod update_currency_v1;
mod update_oracle_v1;

pub(crate) use self::create_assertion_v1::*;
pub(crate) use self::create_currency_v1::*;
pub(crate) use self::create_oracle_v1::*;
pub(crate) use self::create_request_v1::*;
pub(crate) use self::resolve_assertion_v1::*;
pub(crate) use self::update_currency_v1::*;
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
        I::UpdateCurrencyV1(args) => update_currency_v1(program_id, accounts, args),
        I::CreateRequestV1(args) => create_request_v1(program_id, accounts, args),
        I::CreateAssertionV1(args) => create_assertion_v1(program_id, accounts, args),
        I::ResolveAssertionV1 => resolve_assertion_v1(program_id, accounts),

        I::DisputeAssertion(args) => assertion::dispute(program_id, accounts, args),
        I::SubmitVote(args) => voting::submit(program_id, accounts, args),
        I::FinalizeVoting(args) => voting::finalize(program_id, accounts, args),
    }
}
