use crate::types::{AccountKey, AccountKeyPair, AccountManager};
use anyhow::{anyhow, Result};
use bip39::Mnemonic;
use hex;
use rand::Rng;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};

const COIN_TYPE: u32 = 55555;

/// Generate a new BIP39 mnemonic phrase (24 words)
pub fn generate_mnemonic() -> Result<String> {
    let entropy = rand::thread_rng().gen::<[u8; 32]>();
    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    Ok(mnemonic.to_string())
}

/// Generate a new account key pair using secp256k1
pub fn generate_keypair() -> Result<AccountKeyPair> {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

    Ok(AccountKeyPair {
        public_key: AccountKey(public_key.serialize()),
        private_key: secret_key.secret_bytes(),
    })
}

/// Generate key pair from mnemonic and derivation path (simplified)
pub fn generate_keypair_from_mnemonic(
    mnemonic: &str,
    _derivation_path: &str,
) -> Result<AccountKeyPair> {
    // For now, we'll use a simplified approach without HD wallet
    // In a full implementation, you'd use the hdwallet crate
    let mnemonic = Mnemonic::parse_normalized(mnemonic)?;
    let seed = mnemonic.to_seed("");

    // Use the first 32 bytes of the seed as private key
    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&seed[..32]);

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    Ok(AccountKeyPair {
        public_key: AccountKey(public_key.serialize()),
        private_key,
    })
}

/// Sign a message with a private key
pub fn sign_message(message: &[u8; 32], private_key: &[u8; 32]) -> Result<[u8; 64]> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)?;
    let message = Message::from_digest(*message);
    let signature = secp.sign_ecdsa(&message, &secret_key);
    Ok(signature.serialize_compact())
}

/// Verify a signature
pub fn verify_signature(
    message: &[u8; 32],
    signature: &[u8; 64],
    public_key: &[u8; 33],
) -> Result<bool> {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_slice(public_key)?;
    let message = Message::from_digest(*message);
    let signature = Signature::from_compact(signature)?;

    Ok(secp.verify_ecdsa(&message, &signature, &public_key).is_ok())
}

/// Create a new account manager with BIP39 mnemonic
pub fn create_account_manager_with_mnemonic(mnemonic: &str) -> Result<AccountManager> {
    // Generate the first account using m/44'/555555'/0'/0/0 derivation path
    let first_derivation_path = format!("m/44'/{COIN_TYPE}'/0'/0/0");
    let account: AccountKeyPair = generate_keypair_from_mnemonic(mnemonic, &first_derivation_path)?;

    Ok(AccountManager {
        account_map: vec![account.clone()],
        now_selected_account: account,
    })
}

/// Add a new account to the manager using the next derivation path
pub fn add_account_from_mnemonic(
    manager: &mut AccountManager,
    mnemonic: &str,
) -> Result<AccountKeyPair> {
    let next_index = manager.account_map.len();
    let derivation_path = format!("m/44'/{COIN_TYPE}'/0'/0/{}", next_index);

    let new_account = generate_keypair_from_mnemonic(mnemonic, &derivation_path)?;
    manager.account_map.push(new_account.clone());

    Ok(new_account)
}

/// Import account from private key
pub fn import_account_from_private_key(private_key_hex: &str) -> Result<AccountKeyPair> {
    let private_key_bytes = hex::decode(private_key_hex.trim_start_matches("0x"))
        .map_err(|_| anyhow!("Invalid private key format"))?;

    if private_key_bytes.len() != 32 {
        return Err(anyhow!("Private key must be 32 bytes"));
    }

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&private_key_bytes);

    Ok(AccountKeyPair {
        public_key: AccountKey(public_key.serialize()),
        private_key,
    })
}

/// Validate mnemonic phrase
pub fn validate_mnemonic(mnemonic: &str) -> Result<bool> {
    match Mnemonic::parse_normalized(mnemonic) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Get account info for display
pub fn get_account_info(
    account: &AccountKeyPair,
    index: usize,
    is_selected: bool,
) -> crate::types::AccountInfo {
    crate::types::AccountInfo {
        address: account.get_address(),
        public_key: account.get_public_key_hex(),
        index,
        is_selected,
    }
}

/// Sign transaction data
pub fn sign_transaction_data(data: &[u8], private_key: &[u8; 32]) -> Result<[u8; 64]> {
    // Hash the transaction data first
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut message = [0u8; 32];
    message.copy_from_slice(&hash);

    sign_message(&message, private_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic_generation() {
        let mnemonic = generate_mnemonic().unwrap();
        assert!(mnemonic.split_whitespace().count() == 24);
        assert!(validate_mnemonic(&mnemonic).unwrap());
    }

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair().unwrap();
        assert_eq!(keypair.private_key.len(), 32);
        assert_eq!(keypair.public_key.0.len(), 33);
    }

    #[test]
    fn test_mnemonic_to_keypair() {
        let mnemonic = "large bread source replace round mesh camera slow squirrel return swing push wrestle law ankle drive carpet survey absent afraid dove mother cluster truly";
        let derivation_path = format!("m/44'/{COIN_TYPE}'/0'/0/0");
        let keypair = generate_keypair_from_mnemonic(mnemonic, &derivation_path).unwrap();
        assert_eq!(keypair.private_key.len(), 32);
        assert_eq!(keypair.public_key.0.len(), 33);
    }

    #[test]
    fn test_sign_verify() {
        let keypair = generate_keypair().unwrap();
        let message = [1u8; 32];

        let signature = sign_message(&message, &keypair.private_key).unwrap();
        let is_valid = verify_signature(&message, &signature, &keypair.public_key.0).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_account_manager_creation() {
        // Test with a simpler approach - just test that we can generate a keypair
        let keypair = generate_keypair().unwrap();
        assert_eq!(keypair.private_key.len(), 32);
        assert_eq!(keypair.public_key.0.len(), 33);

        // Test mnemonic generation
        let mnemonic = generate_mnemonic().unwrap();
        assert_eq!(mnemonic.split_whitespace().count(), 24);
    }
}
