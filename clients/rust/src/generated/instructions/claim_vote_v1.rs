//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ClaimVoteV1 {
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Assertion
    pub assertion: solana_program::pubkey::Pubkey,
    /// Voting
    pub voting: solana_program::pubkey::Pubkey,
    /// Vote
    pub vote: solana_program::pubkey::Pubkey,
    /// Stake
    pub stake: solana_program::pubkey::Pubkey,
    /// Bond mint
    pub bond_mint: solana_program::pubkey::Pubkey,
    /// Bond destination token account
    pub bond_destination: solana_program::pubkey::Pubkey,
    /// Bond escrow token account of incorrect asserter/disputer
    pub bond_escrow: solana_program::pubkey::Pubkey,
    /// Voter
    pub voter: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl ClaimVoteV1 {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.request, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.assertion, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.voting, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.vote, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.stake, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.bond_mint, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.bond_destination, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.bond_escrow, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.voter, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ClaimVoteV1InstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ClaimVoteV1InstructionData {
    discriminator: u8,
}

impl ClaimVoteV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 15 }
    }
}

impl Default for ClaimVoteV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ClaimVoteV1`.
///
/// ### Accounts:
///
///   0. `[]` request
///   1. `[]` assertion
///   2. `[]` voting
///   3. `[writable]` vote
///   4. `[]` stake
///   5. `[]` bond_mint
///   6. `[writable]` bond_destination
///   7. `[writable]` bond_escrow
///   8. `[writable, signer]` voter
///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   10. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct ClaimVoteV1Builder {
    request: Option<solana_program::pubkey::Pubkey>,
    assertion: Option<solana_program::pubkey::Pubkey>,
    voting: Option<solana_program::pubkey::Pubkey>,
    vote: Option<solana_program::pubkey::Pubkey>,
    stake: Option<solana_program::pubkey::Pubkey>,
    bond_mint: Option<solana_program::pubkey::Pubkey>,
    bond_destination: Option<solana_program::pubkey::Pubkey>,
    bond_escrow: Option<solana_program::pubkey::Pubkey>,
    voter: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClaimVoteV1Builder {
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
    /// Voting
    #[inline(always)]
    pub fn voting(&mut self, voting: solana_program::pubkey::Pubkey) -> &mut Self {
        self.voting = Some(voting);
        self
    }
    /// Vote
    #[inline(always)]
    pub fn vote(&mut self, vote: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vote = Some(vote);
        self
    }
    /// Stake
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// Bond mint
    #[inline(always)]
    pub fn bond_mint(&mut self, bond_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bond_mint = Some(bond_mint);
        self
    }
    /// Bond destination token account
    #[inline(always)]
    pub fn bond_destination(
        &mut self,
        bond_destination: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.bond_destination = Some(bond_destination);
        self
    }
    /// Bond escrow token account of incorrect asserter/disputer
    #[inline(always)]
    pub fn bond_escrow(&mut self, bond_escrow: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bond_escrow = Some(bond_escrow);
        self
    }
    /// Voter
    #[inline(always)]
    pub fn voter(&mut self, voter: solana_program::pubkey::Pubkey) -> &mut Self {
        self.voter = Some(voter);
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
        let accounts = ClaimVoteV1 {
            request: self.request.expect("request is not set"),
            assertion: self.assertion.expect("assertion is not set"),
            voting: self.voting.expect("voting is not set"),
            vote: self.vote.expect("vote is not set"),
            stake: self.stake.expect("stake is not set"),
            bond_mint: self.bond_mint.expect("bond_mint is not set"),
            bond_destination: self.bond_destination.expect("bond_destination is not set"),
            bond_escrow: self.bond_escrow.expect("bond_escrow is not set"),
            voter: self.voter.expect("voter is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `claim_vote_v1` CPI accounts.
pub struct ClaimVoteV1CpiAccounts<'a, 'b> {
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voting
    pub voting: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vote
    pub vote: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond mint
    pub bond_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond destination token account
    pub bond_destination: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond escrow token account of incorrect asserter/disputer
    pub bond_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voter
    pub voter: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `claim_vote_v1` CPI instruction.
pub struct ClaimVoteV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Assertion
    pub assertion: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voting
    pub voting: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vote
    pub vote: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond mint
    pub bond_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond destination token account
    pub bond_destination: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond escrow token account of incorrect asserter/disputer
    pub bond_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    /// Voter
    pub voter: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ClaimVoteV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClaimVoteV1CpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            request: accounts.request,
            assertion: accounts.assertion,
            voting: accounts.voting,
            vote: accounts.vote,
            stake: accounts.stake,
            bond_mint: accounts.bond_mint,
            bond_destination: accounts.bond_destination,
            bond_escrow: accounts.bond_escrow,
            voter: accounts.voter,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.assertion.key,
            false,
        ));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.voting.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.vote.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.stake.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bond_mint.key,
            false,
        ));
        accounts
            .push(solana_program::instruction::AccountMeta::new(*self.bond_destination.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.bond_escrow.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.voter.key, true));
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
        let data = ClaimVoteV1InstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.assertion.clone());
        account_infos.push(self.voting.clone());
        account_infos.push(self.vote.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.bond_mint.clone());
        account_infos.push(self.bond_destination.clone());
        account_infos.push(self.bond_escrow.clone());
        account_infos.push(self.voter.clone());
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

/// Instruction builder for `ClaimVoteV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` request
///   1. `[]` assertion
///   2. `[]` voting
///   3. `[writable]` vote
///   4. `[]` stake
///   5. `[]` bond_mint
///   6. `[writable]` bond_destination
///   7. `[writable]` bond_escrow
///   8. `[writable, signer]` voter
///   9. `[]` token_program
///   10. `[]` system_program
#[derive(Clone, Debug)]
pub struct ClaimVoteV1CpiBuilder<'a, 'b> {
    instruction: Box<ClaimVoteV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimVoteV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimVoteV1CpiBuilderInstruction {
            __program: program,
            request: None,
            assertion: None,
            voting: None,
            vote: None,
            stake: None,
            bond_mint: None,
            bond_destination: None,
            bond_escrow: None,
            voter: None,
            token_program: None,
            system_program: None,
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
    /// Voting
    #[inline(always)]
    pub fn voting(
        &mut self,
        voting: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.voting = Some(voting);
        self
    }
    /// Vote
    #[inline(always)]
    pub fn vote(&mut self, vote: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vote = Some(vote);
        self
    }
    /// Stake
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Bond mint
    #[inline(always)]
    pub fn bond_mint(
        &mut self,
        bond_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bond_mint = Some(bond_mint);
        self
    }
    /// Bond destination token account
    #[inline(always)]
    pub fn bond_destination(
        &mut self,
        bond_destination: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bond_destination = Some(bond_destination);
        self
    }
    /// Bond escrow token account of incorrect asserter/disputer
    #[inline(always)]
    pub fn bond_escrow(
        &mut self,
        bond_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bond_escrow = Some(bond_escrow);
        self
    }
    /// Voter
    #[inline(always)]
    pub fn voter(&mut self, voter: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.voter = Some(voter);
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
        let instruction = ClaimVoteV1Cpi {
            __program: self.instruction.__program,

            request: self.instruction.request.expect("request is not set"),

            assertion: self.instruction.assertion.expect("assertion is not set"),

            voting: self.instruction.voting.expect("voting is not set"),

            vote: self.instruction.vote.expect("vote is not set"),

            stake: self.instruction.stake.expect("stake is not set"),

            bond_mint: self.instruction.bond_mint.expect("bond_mint is not set"),

            bond_destination: self
                .instruction
                .bond_destination
                .expect("bond_destination is not set"),

            bond_escrow: self.instruction.bond_escrow.expect("bond_escrow is not set"),

            voter: self.instruction.voter.expect("voter is not set"),

            token_program: self.instruction.token_program.expect("token_program is not set"),

            system_program: self.instruction.system_program.expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ClaimVoteV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    assertion: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    voting: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vote: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bond_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bond_destination: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bond_escrow: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    voter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
