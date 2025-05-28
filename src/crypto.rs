use crate::LicenseError;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ring::{
    rand::SystemRandom,
    signature::{
        self, EcdsaKeyPair, KeyPair as RingKeyPair,
        ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED,
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let key_pair = KeyPair::generate().unwrap();
        assert!(!key_pair.public_key().is_empty());
        assert!(!key_pair.private_key().is_empty());
    }

    #[test]
    fn test_signature() {
        let key_pair = KeyPair::generate().unwrap();
        let message = b"test message";
        let signature = key_pair.sign(message).unwrap();
        
        // Verify with same key pair
        KeyPair::verify(key_pair.public_key(), message, &signature).unwrap();
        
        // Verify with wrong message should fail
        assert!(KeyPair::verify(key_pair.public_key(), b"wrong message", &signature).is_err());
    }

    #[test]
    fn test_key_pair_from_private_key() {
        let key_pair1 = KeyPair::generate().unwrap();
        let private_key = key_pair1.private_key().to_vec();
        let key_pair2 = KeyPair::from_private_key(&private_key).unwrap();
        
        assert_eq!(key_pair1.public_key(), key_pair2.public_key());
    }
}

pub struct KeyPair {
    key_pair: EcdsaKeyPair,
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl KeyPair {
    pub fn generate() -> Result<Self, LicenseError> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .map_err(|_| LicenseError::InvalidSignature)?;

        Self::from_private_key(pkcs8_bytes.as_ref())
    }

    pub fn from_private_key(pkcs8_bytes: &[u8]) -> Result<Self, LicenseError> {
        let rng = SystemRandom::new();
        let key_pair = EcdsaKeyPair::from_pkcs8(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            pkcs8_bytes,
            &rng,
        )
        .map_err(|_| LicenseError::InvalidSignature)?;

        let public_key = key_pair.public_key().as_ref().to_vec();

        Ok(Self {
            key_pair,
            public_key,
            private_key: pkcs8_bytes.to_vec(),
        })
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, LicenseError> {
        let rng = SystemRandom::new();
        self.key_pair
            .sign(&rng, data)
            .map(|sig| sig.as_ref().to_vec())
            .map_err(|_| LicenseError::InvalidSignature)
    }

    pub fn verify(public_key: &[u8], data: &[u8], signature: &[u8]) -> Result<(), LicenseError> {
        let public_key = signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
        public_key
            .verify(data, signature)
            .map_err(|_| LicenseError::InvalidSignature)
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }
}

pub fn base64_encode(data: &[u8]) -> String {
    STANDARD.encode(data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, LicenseError> {
    STANDARD.decode(data).map_err(|_| LicenseError::InvalidFormat)
}