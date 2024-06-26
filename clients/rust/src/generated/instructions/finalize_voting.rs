//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::FinalizeVotingArgs;
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct FinalizeVoting {
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Voting
    pub voting: solana_program::pubkey::Pubkey,
}

impl FinalizeVoting {
    pub fn instruction(
        &self,
        args: FinalizeVotingInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: FinalizeVotingInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(self.request, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.voting, false));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = FinalizeVotingInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct FinalizeVotingInstructionData {
    discriminator: u8,
}

impl FinalizeVotingInstructionData {
    fn new() -> Self {
        Self { discriminator: 6 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinalizeVotingInstructionArgs {
    pub finalize_voting_args: FinalizeVotingArgs,
}

/// Instruction builder for `FinalizeVoting`.
///
/// ### Accounts:
///
///   0. `[writable]` request
///   1. `[writable]` voting
#[derive(Default)]
pub struct FinalizeVotingBuilder {
    request: Option<solana_program::pubkey::Pubkey>,
    voting: Option<solana_program::pubkey::Pubkey>,
    finalize_voting_args: Option<FinalizeVotingArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl FinalizeVotingBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// Voting
    #[inline(always)]
    pub fn voting(&mut self, voting: solana_program::pubkey::Pubkey) -> &mut Self {
        self.voting = Some(voting);
        self
    }
    #[inline(always)]
    pub fn finalize_voting_args(&mut self, finalize_voting_args: FinalizeVotingArgs) -> &mut Self {
        self.finalize_voting_args = Some(finalize_voting_args);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = FinalizeVoting {
            request: self.request.expect("request is not set"),
            voting: self.voting.expect("voting is not set"),
        };
        let args = FinalizeVotingInstructionArgs {
            finalize_voting_args: self
                .finalize_voting_args
                .clone()
                .expect("finalize_voting_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `finalize_voting` CPI accounts.
pub struct FinalizeVotingCpiAccounts<'a, 'b> {
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voting
    pub voting: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `finalize_voting` CPI instruction.
pub struct FinalizeVotingCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voting
    pub voting: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: FinalizeVotingInstructionArgs,
}

impl<'a, 'b> FinalizeVotingCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: FinalizeVotingCpiAccounts<'a, 'b>,
        args: FinalizeVotingInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            request: accounts.request,
            voting: accounts.voting,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.voting.key, false));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = FinalizeVotingInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.voting.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `FinalizeVoting` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` request
///   1. `[writable]` voting
pub struct FinalizeVotingCpiBuilder<'a, 'b> {
    instruction: Box<FinalizeVotingCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> FinalizeVotingCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(FinalizeVotingCpiBuilderInstruction {
            __program: program,
            request: None,
            voting: None,
            finalize_voting_args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Request
    #[inline(always)]
    pub fn request(
        &mut self,
        request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.request = Some(request);
        self
    }
    /// Voting
    #[inline(always)]
    pub fn voting(
        &mut self,
        voting: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.voting = Some(voting);
        self
    }
    #[inline(always)]
    pub fn finalize_voting_args(&mut self, finalize_voting_args: FinalizeVotingArgs) -> &mut Self {
        self.instruction.finalize_voting_args = Some(finalize_voting_args);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = FinalizeVotingInstructionArgs {
            finalize_voting_args: self
                .instruction
                .finalize_voting_args
                .clone()
                .expect("finalize_voting_args is not set"),
        };
        let instruction = FinalizeVotingCpi {
            __program: self.instruction.__program,

            request: self.instruction.request.expect("request is not set"),

            voting: self.instruction.voting.expect("voting is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct FinalizeVotingCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    voting: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    finalize_voting_args: Option<FinalizeVotingArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
