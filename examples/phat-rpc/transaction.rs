use crate::era::Era;
use ink_env::AccountId;
use ink_prelude::{string::String, vec::Vec};
use scale::{Compact, Decode, Encode, Error, Input, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct UnsignedExtrinsic<Call> {
    pub pallet_id: u8,
    pub call_id: u8,
    pub call: Call,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Remark {
    pub remark: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
struct Transfer {
    dest: MultiAddress<AccountId, u32>,
    currency_id: CurrencyId,
    amount: Compact<u128>,
}

#[derive(Encode, Decode, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Extra {
    // 0 if Immortal, or Vec<u64, u64> for period and the phase.
    era: Era,
    // Nonce
    nonce: Compact<u64>,
    // Tip for the block producer.
    tip: Compact<u128>,
}

/// A multi-format address wrapper for on-chain accounts.
#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum MultiAddress<AccountId, AccountIndex> {
    /// It's an account ID (pubkey).
    Id(AccountId),
    /// It's an account index.
    Index(#[codec(compact)] AccountIndex),
    /// It's some arbitrary raw bytes.
    Raw(Vec<u8>),
    /// It's a 32 byte representation.
    Address32([u8; 32]),
    /// Its a 20 byte representation.
    Address20([u8; 20]),
}

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> {
    fn from(a: AccountId) -> Self {
        Self::Id(a)
    }
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum MultiSignature<Signature> {
    /// An Ed25519 signature.
    Ed25519(Signature),
    /// An Sr25519 signature.
    Sr25519(Signature),
    /// An ECDSA/SECP256k1 signature.
    Ecdsa(Signature),
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CurrencyId {
    FREN,
    GM,
    GN,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SignedExtrinsic {
    address: MultiAddress<AccountId, u32>,
    signature: Vec<u8>,
    extra: Extra,
    call: UnsignedExtrinsic<Transfer>,
}