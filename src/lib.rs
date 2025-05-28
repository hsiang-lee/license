use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::crypto::KeyPair;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, TimeDelta};

    #[test]
    fn test_license_creation() {
        let expires = Utc::now() + TimeDelta::days(30);
        let license = License::new(
            "customer1".to_string(),
            "product1".to_string(),
            expires,
            vec!["feature1".to_string()],
            Some("machine123".to_string()),
        );
        assert_eq!(license.product_id, "product1");
        assert_eq!(license.machine_id, Some("machine123".to_string()));
    }

    #[test]
    fn test_license_signature() {
        let expires = Utc::now() + TimeDelta::days(30);
        let mut license = License::new(
            "customer1".to_string(),
            "product1".to_string(),
            expires,
            vec!["feature1".to_string()],
            Some("machine123".to_string()),
        );
        let key_pair = crypto::KeyPair::generate().unwrap();
        license.sign(&key_pair).unwrap();
        assert!(license.validate(key_pair.public_key()).is_ok());
    }
}

#[derive(Error, Debug)]
pub enum LicenseError {
    #[error("Invalid license format")]
    InvalidFormat,
    #[error("License expired")]
    Expired,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Machine ID mismatch")]
    MachineIdMismatch,
    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for LicenseError {
    fn from(err: std::io::Error) -> Self {
        LicenseError::IoError(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub customer_id: String,
    pub product_id: String,
    pub expiry_date: String,
    pub features: Vec<String>,
    pub machine_id: Option<String>,
    #[serde(skip)]
    pub signature: Option<String>,
}

impl License {
    pub fn new(
        customer_id: String,
        product_id: String,
        expiry_date: DateTime<Utc>,
        features: Vec<String>,
        machine_id: Option<String>,
    ) -> Self {
        Self {
            customer_id,
            product_id,
            expiry_date: expiry_date.to_rfc3339(),
            features,
            machine_id,
            signature: None,
        }
    }

    pub fn sign(&mut self, key_pair: &KeyPair) -> Result<(), LicenseError> {
        let data = serde_json::to_vec(self).map_err(|_| LicenseError::InvalidFormat)?;
        let signature = key_pair.sign(&data)?;
        self.signature = Some(crypto::base64_encode(&signature));
        Ok(())
    }

    pub fn validate(&self, public_key: &[u8]) -> Result<(), LicenseError> {
        // Check expiry
        let expiry = DateTime::parse_from_rfc3339(&self.expiry_date)
            .map_err(|_| LicenseError::InvalidFormat)?;
        if expiry < Utc::now() {
            return Err(LicenseError::Expired);
        }

        // Check machine binding
        if let Some(required_id) = &self.machine_id {
            let current_id = machine_info::create_provider()?
                .get_machine_id()?;
            if required_id != &current_id {
                return Err(LicenseError::MachineIdMismatch);
            }
        }

        // Verify signature
        let signature = self.signature.as_ref()
            .ok_or(LicenseError::InvalidSignature)?;
        let sig_bytes = crypto::base64_decode(signature)?;
        let mut data = self.clone();
        data.signature = None;
        let data_bytes = serde_json::to_vec(&data)
            .map_err(|_| LicenseError::InvalidFormat)?;

        crypto::KeyPair::verify(public_key, &data_bytes, &sig_bytes)
    }

    pub fn to_string(&self) -> Result<String, LicenseError> {
        serde_json::to_string(self).map_err(|_| LicenseError::InvalidFormat)
    }

    pub fn from_str(data: &str) -> Result<Self, LicenseError> {
        serde_json::from_str(data).map_err(|_| LicenseError::InvalidFormat)
    }
}

pub mod machine_info;
pub mod crypto;
pub mod platform;