//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct CreateStakeV1 {
    /// Oracle
    pub oracle: solana_program::pubkey::Pubkey,
    /// Stake
    pub stake: solana_program::pubkey::Pubkey,
    /// Governance token mint
    pub mint: solana_program::pubkey::Pubkey,
    /// Stake source token account
    pub stake_source: solana_program::pubkey::Pubkey,
    /// Stake pool token account
    pub stake_pool: solana_program::pubkey::Pubkey,
    /// Stake owner
    pub wallet: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateStakeV1 {
    pub fn instruction(
        &self,
        args: CreateStakeV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateStakeV1InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.oracle, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.stake, true));
        accounts.push(solana_program::instruction::AccountMeta::new(self.mint, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.stake_source, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.stake_pool, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.wallet, true));
        accounts.push(solana_program::instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateStakeV1InstructionData::new().try_to_vec().unwrap();
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
pub struct CreateStakeV1InstructionData {
    discriminator: u8,
}

impl CreateStakeV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 12 }
    }
}

impl Default for CreateStakeV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateStakeV1InstructionArgs {
    pub amount: u64,
}

/// Instruction builder for `CreateStakeV1`.
///
/// ### Accounts:
///
///   0. `[]` oracle
///   1. `[writable, signer]` stake
///   2. `[writable]` mint
///   3. `[writable]` stake_source
///   4. `[writable]` stake_pool
///   5. `[writable, signer]` wallet
///   6. `[writable, signer]` payer
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateStakeV1Builder {
    oracle: Option<solana_program::pubkey::Pubkey>,
    stake: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    stake_source: Option<solana_program::pubkey::Pubkey>,
    stake_pool: Option<solana_program::pubkey::Pubkey>,
    wallet: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateStakeV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Oracle
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }
    /// Stake
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// Governance token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// Stake source token account
    #[inline(always)]
    pub fn stake_source(&mut self, stake_source: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_source = Some(stake_source);
        self
    }
    /// Stake pool token account
    #[inline(always)]
    pub fn stake_pool(&mut self, stake_pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_pool = Some(stake_pool);
        self
    }
    /// Stake owner
    #[inline(always)]
    pub fn wallet(&mut self, wallet: solana_program::pubkey::Pubkey) -> &mut Self {
        self.wallet = Some(wallet);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL token program
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
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
        let accounts = CreateStakeV1 {
            oracle: self.oracle.expect("oracle is not set"),
            stake: self.stake.expect("stake is not set"),
            mint: self.mint.expect("mint is not set"),
            stake_source: self.stake_source.expect("stake_source is not set"),
            stake_pool: self.stake_pool.expect("stake_pool is not set"),
            wallet: self.wallet.expect("wallet is not set"),
            payer: self.payer.expect("payer is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateStakeV1InstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_stake_v1` CPI accounts.
pub struct CreateStakeV1CpiAccounts<'a, 'b> {
    /// Oracle
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Governance token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake source token account
    pub stake_source: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake pool token account
    pub stake_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake owner
    pub wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_stake_v1` CPI instruction.
pub struct CreateStakeV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Governance token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake source token account
    pub stake_source: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake pool token account
    pub stake_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake owner
    pub wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateStakeV1InstructionArgs,
}

impl<'a, 'b> CreateStakeV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateStakeV1CpiAccounts<'a, 'b>,
        args: CreateStakeV1InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            oracle: accounts.oracle,
            stake: accounts.stake,
            mint: accounts.mint,
            stake_source: accounts.stake_source,
            stake_pool: accounts.stake_pool,
            wallet: accounts.wallet,
            payer: accounts.payer,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.oracle.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.stake.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.mint.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.stake_source.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.stake_pool.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.wallet.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateStakeV1InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.stake_source.clone());
        account_infos.push(self.stake_pool.clone());
        account_infos.push(self.wallet.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `CreateStakeV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` oracle
///   1. `[writable, signer]` stake
///   2. `[writable]` mint
///   3. `[writable]` stake_source
///   4. `[writable]` stake_pool
///   5. `[writable, signer]` wallet
///   6. `[writable, signer]` payer
///   7. `[]` token_program
///   8. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateStakeV1CpiBuilder<'a, 'b> {
    instruction: Box<CreateStakeV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateStakeV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateStakeV1CpiBuilderInstruction {
            __program: program,
            oracle: None,
            stake: None,
            mint: None,
            stake_source: None,
            stake_pool: None,
            wallet: None,
            payer: None,
            token_program: None,
            system_program: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Oracle
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }
    /// Stake
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Governance token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// Stake source token account
    #[inline(always)]
    pub fn stake_source(
        &mut self,
        stake_source: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_source = Some(stake_source);
        self
    }
    /// Stake pool token account
    #[inline(always)]
    pub fn stake_pool(
        &mut self,
        stake_pool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_pool = Some(stake_pool);
        self
    }
    /// Stake owner
    #[inline(always)]
    pub fn wallet(
        &mut self,
        wallet: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.wallet = Some(wallet);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// SPL token program
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
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
        let args = CreateStakeV1InstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = CreateStakeV1Cpi {
            __program: self.instruction.__program,

            oracle: self.instruction.oracle.expect("oracle is not set"),

            stake: self.instruction.stake.expect("stake is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            stake_source: self.instruction.stake_source.expect("stake_source is not set"),

            stake_pool: self.instruction.stake_pool.expect("stake_pool is not set"),

            wallet: self.instruction.wallet.expect("wallet is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            token_program: self.instruction.token_program.expect("token_program is not set"),

            system_program: self.instruction.system_program.expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateStakeV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    wallet: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
