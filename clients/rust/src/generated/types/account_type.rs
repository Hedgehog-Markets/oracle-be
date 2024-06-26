//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;

#[derive(
    BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountType {
    Uninitialized,
    OracleV1,
    ConfigV1,
    StakeV1,
    RequestV1,
    AssertionV1,
    CurrencyV1,
    VotingV1,
    VoteV1,
}
