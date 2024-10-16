//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::AccountType;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigV1 {
    pub account_type: AccountType,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub authority: Pubkey,
    pub bond_fee_bps: u16,
    pub dispute_window: u32,
    pub voting_window: u32,
    pub arbitration_window: u32,
}

impl ConfigV1 {
    pub const LEN: usize = 47;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for ConfigV1 {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for ConfigV1 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for ConfigV1 {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for ConfigV1 {
    fn owner() -> Pubkey {
        crate::OPTIMISTIC_ORACLE_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for ConfigV1 {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for ConfigV1 {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
