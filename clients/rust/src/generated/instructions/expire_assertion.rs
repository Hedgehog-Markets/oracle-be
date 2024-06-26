//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::ExpireAssertionArgs;
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ExpireAssertion {
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Assertion
    pub assertion: solana_program::pubkey::Pubkey,
}

impl ExpireAssertion {
    pub fn instruction(
        &self,
        args: ExpireAssertionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ExpireAssertionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(self.request, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.assertion, false));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = ExpireAssertionInstructionData::new().try_to_vec().unwrap();
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
struct ExpireAssertionInstructionData {
    discriminator: u8,
}

impl ExpireAssertionInstructionData {
    fn new() -> Self {
        Self { discriminator: 3 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpireAssertionInstructionArgs {
    pub expire_assertion_args: ExpireAssertionArgs,
}

/// Instruction builder for `ExpireAssertion`.
///
/// ### Accounts:
///
///   0. `[writable]` request
///   1. `[]` assertion
#[derive(Default)]
pub struct ExpireAssertionBuilder {
    request: Option<solana_program::pubkey::Pubkey>,
    assertion: Option<solana_program::pubkey::Pubkey>,
    expire_assertion_args: Option<ExpireAssertionArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ExpireAssertionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// Assertion
    #[inline(always)]
    pub fn assertion(&mut self, assertion: solana_program::pubkey::Pubkey) -> &mut Self {
        self.assertion = Some(assertion);
        self
    }
    #[inline(always)]
    pub fn expire_assertion_args(
        &mut self,
        expire_assertion_args: ExpireAssertionArgs,
    ) -> &mut Self {
        self.expire_assertion_args = Some(expire_assertion_args);
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
        let accounts = ExpireAssertion {
            request: self.request.expect("request is not set"),
            assertion: self.assertion.expect("assertion is not set"),
        };
        let args = ExpireAssertionInstructionArgs {
            expire_assertion_args: self
                .expire_assertion_args
                .clone()
                .expect("expire_assertion_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `expire_assertion` CPI accounts.
pub struct ExpireAssertionCpiAccounts<'a, 'b> {
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `expire_assertion` CPI instruction.
pub struct ExpireAssertionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ExpireAssertionInstructionArgs,
}

impl<'a, 'b> ExpireAssertionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ExpireAssertionCpiAccounts<'a, 'b>,
        args: ExpireAssertionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            request: accounts.request,
            assertion: accounts.assertion,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.assertion.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = ExpireAssertionInstructionData::new().try_to_vec().unwrap();
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
        account_infos.push(self.assertion.clone());
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

/// Instruction builder for `ExpireAssertion` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` request
///   1. `[]` assertion
pub struct ExpireAssertionCpiBuilder<'a, 'b> {
    instruction: Box<ExpireAssertionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ExpireAssertionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ExpireAssertionCpiBuilderInstruction {
            __program: program,
            request: None,
            assertion: None,
            expire_assertion_args: None,
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
    /// Assertion
    #[inline(always)]
    pub fn assertion(
        &mut self,
        assertion: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.assertion = Some(assertion);
        self
    }
    #[inline(always)]
    pub fn expire_assertion_args(
        &mut self,
        expire_assertion_args: ExpireAssertionArgs,
    ) -> &mut Self {
        self.instruction.expire_assertion_args = Some(expire_assertion_args);
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
        let args = ExpireAssertionInstructionArgs {
            expire_assertion_args: self
                .instruction
                .expire_assertion_args
                .clone()
                .expect("expire_assertion_args is not set"),
        };
        let instruction = ExpireAssertionCpi {
            __program: self.instruction.__program,

            request: self.instruction.request.expect("request is not set"),

            assertion: self.instruction.assertion.expect("assertion is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ExpireAssertionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    assertion: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    expire_assertion_args: Option<ExpireAssertionArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
