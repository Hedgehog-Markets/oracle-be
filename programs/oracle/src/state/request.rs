use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use borsh_size::BorshSize;
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::pda;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize, ShankAccount)]
pub struct RequestV1 {
    account_type: AccountType,

    /// Index of the request in the oracle.
    pub index: u64,

    /// Config address.
    pub config: Pubkey,
    /// Creator address.
    pub creator: Pubkey,

    /// Amount rewarded to the asserter/disputer on resolution.
    pub reward: u64,
    /// Reward mint.
    pub reward_mint: Pubkey,

    /// Amount required to be bonded by asserter/disputer.
    pub bond: u64,
    /// Bond mint.
    pub bond_mint: Pubkey,

    /// Unix timestamp after which a value can be asserted.
    pub assertion_timestamp: i64,
    /// Unix timestamp at which the request was resolved.
    pub resolve_timestamp: i64,

    /// Request state.
    pub state: RequestState,
    /// The round of the request.
    pub round: u8,
    /// Value of the resolved request.
    pub value: u64,

    /// Arbitrator address.
    ///
    /// The arbitrator has the ability to override the result of voting. This
    /// takes the form of a window after voting in which the result can be
    /// changed.
    ///
    /// If the address is the default pubkey (`11111111111111111111111111111111`),
    /// then the request is considered to have no arbitrator.
    pub arbitrator: Pubkey,

    /// The type of data the request is for.
    pub kind: RequestKind,
    /// Off-chain data for the request.
    pub uri: String,
}

#[derive(Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize)]
#[repr(u8)]
pub enum RequestState {
    /// Request pending a proposal.
    Requested,
    /// Request with a asserted value awaiting resolution.
    Asserted,
    /// Request with a disputed value awaiting voting resolution.
    Disputed,
    /// Request with a resolved value.
    Resolved,
}

pub const VALUE_UNAVAILABLE: u64 = 0;

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize)]
pub enum RequestKind {
    /// Yes/No request:
    /// - 1 = Yes
    /// - 2 = No
    /// - 3 = Invalid
    YesNo,
}

impl RequestV1 {
    pub fn has_arbitrator(&self) -> bool {
        const DEFAULT_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]);

        !solana_utils::pubkeys_eq(&self.arbitrator, &DEFAULT_PUBKEY)
    }

    pub fn assert_pda(&self, request: &Pubkey) -> Result<u8, ProgramError> {
        pda::request::assert_pda(request, &self.index)
    }

    pub fn assert_config(&self, config: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.config, config) {
            return Err(OracleError::ConfigMismatch);
        }
        Ok(())
    }

    pub fn assert_reward_mint(&self, mint: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.reward_mint, mint) {
            return Err(OracleError::RewardMintMismatch);
        }
        Ok(())
    }

    pub fn assert_bond_mint(&self, mint: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.bond_mint, mint) {
            return Err(OracleError::BondMintMismatch);
        }
        Ok(())
    }

    pub fn validate_assertion_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if timestamp < self.assertion_timestamp {
            return Err(OracleError::AssertionTooEarly);
        }
        Ok(())
    }
}

impl Account for RequestV1 {
    const TYPE: AccountType = AccountType::RequestV1;
}

impl RequestKind {
    pub fn validate_value(&self, value: u64) -> Result<(), OracleError> {
        if value == VALUE_UNAVAILABLE {
            return Ok(());
        }

        let valid = match self {
            Self::YesNo => matches!(value, 1..=3),
        };

        if valid { Ok(()) } else { Err(OracleError::InvalidValue) }
    }
}

impl TryFrom<InitRequest> for (RequestV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitRequest) -> Result<(RequestV1, usize), Self::Error> {
        let InitRequest {
            index,
            config,
            creator,
            reward,
            reward_mint,
            bond,
            bond_mint,
            timestamp,
            arbitrator,
            kind,
            uri,
        } = params;

        let account = RequestV1 {
            account_type: RequestV1::TYPE,
            index,
            config,
            creator,
            reward,
            reward_mint,
            bond,
            bond_mint,
            assertion_timestamp: timestamp,
            resolve_timestamp: 0,
            state: RequestState::Requested,
            round: 0,
            value: 0,
            arbitrator,
            kind,
            uri,
        };
        let space = account.borsh_size();

        Ok((account, space))
    }
}

pub(crate) struct InitRequest {
    pub index: u64,

    pub config: Pubkey,
    pub creator: Pubkey,

    pub reward: u64,
    pub reward_mint: Pubkey,

    pub bond: u64,
    pub bond_mint: Pubkey,

    pub timestamp: i64,
    pub arbitrator: Pubkey,

    pub kind: RequestKind,
    pub uri: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn account_size() {
        let init = InitRequest {
            index: 0,
            config: Pubkey::new_unique(),
            creator: Pubkey::new_unique(),
            reward: 0,
            reward_mint: Pubkey::new_unique(),
            bond: 0,
            bond_mint: Pubkey::new_unique(),
            timestamp: 0,
            arbitrator: Pubkey::new_unique(),
            kind: RequestKind::YesNo,
            uri: "https://example.com".to_owned(),
        };

        let (request, expected) = <(RequestV1, usize)>::try_from(init).unwrap();
        let actual = borsh::object_length(&request).unwrap();

        assert_eq!(expected, actual);
    }
}
