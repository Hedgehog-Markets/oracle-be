use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct Request {
    account_type: AccountType,

    /// Index of the request in the oracle.
    pub index: u64,

    /// Creator address.
    pub creator: Pubkey,

    /// Amount rewarded to the asserter/disputer on resolution.
    pub reward: u64,
    /// Reward mint.
    pub reward_mint: Pubkey,

    /// Unix timestamp after which a value can be asserted.
    pub timestamp: i64,

    /// Request state.
    pub state: RequestState,
    /// Value of the resolved request.
    pub value: u64,

    // Request data may have varying layouts when serialized. It is at the end
    // of the account to avoid interfering with GPA lookups.
    /// Request data.
    pub data: RequestData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSize)]
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

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum RequestData {
    /// Yes/No request:
    /// - 0 = No
    /// - 1 = Yes
    YesNo {
        /// Question.
        question: String,
    },
}

impl Request {
    const BASE_SIZE: usize =
        AccountType::SIZE       // account_type
        + u64::SIZE             // index
        + Pubkey::SIZE          // creator
        + u64::SIZE             // reward
        + Pubkey::SIZE          // reward_mint
        + i64::SIZE             // timestamp
        + RequestState::SIZE    // state
        + u64::SIZE             // value
        ;
}

impl RequestData {
    fn size(&self) -> Option<usize> {
        let variant_size = match self {
            Self::YesNo { question } => 4usize.checked_add(question.len())?,
        };
        variant_size.checked_add(1)
    }
}

impl Account for Request {
    const TYPE: AccountType = AccountType::Request;
}

impl TryFrom<InitRequest> for (Request, usize) {
    type Error = OracleError;

    fn try_from(params: InitRequest) -> Result<(Request, usize), Self::Error> {
        let InitRequest { index, creator, reward, reward_mint, timestamp, data } = params;

        let space = data
            .size()
            .and_then(|s| s.checked_add(Request::BASE_SIZE))
            .ok_or(OracleError::ArithmeticOverflow)?;

        Ok((
            Request {
                account_type: Request::TYPE,
                index,
                creator,
                reward,
                reward_mint,
                timestamp,
                state: RequestState::Requested,
                value: 0,
                data,
            },
            space,
        ))
    }
}

pub(crate) struct InitRequest {
    pub index: u64,
    pub creator: Pubkey,

    pub reward: u64,
    pub reward_mint: Pubkey,

    pub timestamp: i64,
    pub data: RequestData,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn request_data_size() {
        let data = RequestData::YesNo { question: "example question?".to_owned() };

        let expected = data.size().unwrap();
        let actual = common_test::serialized_len(&data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn request_size() {
        let init = InitRequest {
            creator: Pubkey::new_unique(),
            index: 0,
            reward: 0,
            reward_mint: Pubkey::new_unique(),
            timestamp: 0,
            data: RequestData::YesNo { question: "another example question?".to_owned() },
        };

        let (request, expected) = <(Request, usize)>::try_from(init).unwrap();
        let actual = common_test::serialized_len(&request).unwrap();

        assert_eq!(expected, actual);
    }
}
