//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ResolveAssertionV1 {
    /// Config
    pub config: solana_program::pubkey::Pubkey,
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Assertion
    pub assertion: solana_program::pubkey::Pubkey,
}

impl ResolveAssertionV1 {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.config, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.request, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.assertion, false));
        accounts.extend_from_slice(remaining_accounts);
        let data = ResolveAssertionV1InstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ResolveAssertionV1InstructionData {
    discriminator: u8,
}

impl ResolveAssertionV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 8 }
    }
}

impl Default for ResolveAssertionV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ResolveAssertionV1`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` request
///   2. `[]` assertion
#[derive(Clone, Debug, Default)]
pub struct ResolveAssertionV1Builder {
    config: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    assertion: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ResolveAssertionV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Config
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
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
        let accounts = ResolveAssertionV1 {
            config: self.config.expect("config is not set"),
            request: self.request.expect("request is not set"),
            assertion: self.assertion.expect("assertion is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `resolve_assertion_v1` CPI accounts.
pub struct ResolveAssertionV1CpiAccounts<'a, 'b> {
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `resolve_assertion_v1` CPI instruction.
pub struct ResolveAssertionV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ResolveAssertionV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ResolveAssertionV1CpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            request: accounts.request,
            assertion: accounts.assertion,
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
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.config.key, false));
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
        let data = ResolveAssertionV1InstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
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

/// Instruction builder for `ResolveAssertionV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` request
///   2. `[]` assertion
#[derive(Clone, Debug)]
pub struct ResolveAssertionV1CpiBuilder<'a, 'b> {
    instruction: Box<ResolveAssertionV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ResolveAssertionV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ResolveAssertionV1CpiBuilderInstruction {
            __program: program,
            config: None,
            request: None,
            assertion: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Config
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
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
        let instruction = ResolveAssertionV1Cpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            request: self.instruction.request.expect("request is not set"),

            assertion: self.instruction.assertion.expect("assertion is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ResolveAssertionV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    assertion: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}