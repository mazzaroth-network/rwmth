use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Account key type (33-byte compressed public key)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountKey(pub [u8; 33]);

impl serde::Serialize for AccountKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> serde::Deserialize<'de> for AccountKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 33 {
            return Err(serde::de::Error::custom("Invalid public key length"));
        }
        let mut key = [0u8; 33];
        key.copy_from_slice(&bytes);
        Ok(AccountKey(key))
    }
}

impl AccountKey {
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
        let hex_clean = hex_str.trim_start_matches("0x");
        let bytes = hex::decode(hex_clean)?;
        if bytes.len() != 33 {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut key = [0u8; 33];
        key.copy_from_slice(&bytes);
        Ok(AccountKey(key))
    }
}

/// Account key pair with public and private keys
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountKeyPair {
    pub public_key: AccountKey,
    pub private_key: [u8; 32],
}

impl AccountKeyPair {
    /// Get the address (derived from public key)
    pub fn get_address(&self) -> String {
        // For Mazzaroth, we'll use the first 20 bytes of the SHA256 hash of the public key
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&self.public_key.0);
        let result = hasher.finalize();
        format!("0x{}", hex::encode(&result[..20]))
    }

    /// Get private key as hex string
    pub fn get_private_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.private_key))
    }

    /// Get public key as hex string
    pub fn get_public_key_hex(&self) -> String {
        self.public_key.to_hex()
    }
}

/// Account manager (matches your existing format)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountManager {
    pub account_map: Vec<AccountKeyPair>,
    pub now_selected_account: AccountKeyPair,
}

impl Default for AccountManager {
    fn default() -> Self {
        use crate::crypto::generate_keypair;
        let account = generate_keypair().expect("Failed to generate keypair");
        Self {
            account_map: vec![account.clone()],
            now_selected_account: account,
        }
    }
}

/// Wallet information for CLI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Number of accounts in the wallet
    pub account_count: usize,
    /// Currently selected account address
    pub selected_account: String,
    /// Wallet creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
}

/// Transaction data for signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender address
    pub from: String,
    /// Recipient address
    pub to: Option<String>,
    /// Amount to transfer (in MTH)
    pub amount: Option<u64>,
    /// Transaction data (hex encoded)
    pub data: Option<String>,
    /// Nonce
    pub nonce: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Gas price
    pub gas_price: u64,
}

/// Signature result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// The signature bytes (hex encoded)
    pub signature: String,
    /// The public key (hex encoded)
    pub public_key: String,
    /// The address
    pub address: String,
}

/// Account information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account address
    pub address: String,
    /// Public key
    pub public_key: String,
    /// Account index in the map
    pub index: usize,
    /// Whether this is the selected account
    pub is_selected: bool,
}
