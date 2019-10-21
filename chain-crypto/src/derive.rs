use ed25519_bip32::{PrivateKeyError, XPrv, XPRV_SIZE};
use ed25519_bip32::{DerivationScheme, DerivationError};
use crate::key::{SecretKey, PublicKey};
use crate::{Ed25519Bip32, Ed25519, Ed25519Extended, ExtendedPriv, Pub};

pub fn derive_sk_ed25519(
    key: &SecretKey<Ed25519Bip32>,
    index: u32,
) -> SecretKey<Ed25519Bip32> {
    let new_key = key.0.derive(DerivationScheme::V2, index);
    SecretKey(new_key)
}

pub fn derive_pk_ed25519(
    key: &PublicKey<Ed25519Bip32>,
    index: u32,
) -> Result<PublicKey<Ed25519Bip32>, DerivationError> {
    key.0
      .derive(DerivationScheme::V2, index)
      .map(PublicKey)
}

pub fn to_raw_sk(key: &SecretKey<Ed25519Bip32>) -> SecretKey<Ed25519Extended> {
    SecretKey(ExtendedPriv::from_xprv(&key.0))
}

pub fn to_raw_pk(key: &PublicKey<Ed25519Bip32>) -> PublicKey<Ed25519> {
    PublicKey(Pub::from_xpub(&key.0))
}

pub fn from_bip39_seed(seed: &[u8]) -> Result<SecretKey<Ed25519Bip32>, PrivateKeyError> {
    if seed.len() != XPRV_SIZE {
      return Err(PrivateKeyError::LengthInvalid(seed.len()));
    }

    let mut buf = [0u8; XPRV_SIZE];
    buf[..].clone_from_slice(seed);
    Ok(SecretKey(XPrv::normalize_bytes(buf)))
}
