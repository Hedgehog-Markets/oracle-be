//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::AccountType;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeV1 {
    pub account_type: AccountType,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub mint: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub owner: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub delegate: Pubkey,
    pub amount: u64,
    pub lock_timestamp: i64,
}

impl StakeV1 {
    pub const LEN: usize = 113;

    /// Prefix values used to generate a PDA for this account.
    ///
    /// Values are positional and appear in the following order:
    ///
    ///   0. `StakeV1::PREFIX`
    ///   1. wallet (`Pubkey`)
    pub const PREFIX: &'static [u8] = "stake".as_bytes();

    pub fn create_pda(
        wallet: Pubkey,
        bump: u8,
    ) -> Result<solana_program::pubkey::Pubkey, solana_program::pubkey::PubkeyError> {
        solana_program::pubkey::Pubkey::create_program_address(
            &["stake".as_bytes(), wallet.as_ref(), &[bump]],
            &crate::OPTIMISTIC_ORACLE_ID,
        )
    }

    pub fn find_pda(wallet: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
        solana_program::pubkey::Pubkey::find_program_address(
            &["stake".as_bytes(), wallet.as_ref()],
            &crate::OPTIMISTIC_ORACLE_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for StakeV1 {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for StakeV1 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for StakeV1 {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for StakeV1 {
    fn owner() -> Pubkey {
        crate::OPTIMISTIC_ORACLE_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for StakeV1 {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for StakeV1 {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
