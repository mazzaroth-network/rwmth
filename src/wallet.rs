use crate::{
    crypto::{
        add_account_from_mnemonic, create_account_manager_with_mnemonic, generate_mnemonic,
        get_account_info, sign_transaction_data,
        validate_mnemonic,
    },
    storage::Storage,
    types::{AccountInfo, AccountKeyPair, AccountManager},
};
use anyhow::{anyhow, Result};
use std::path::Path;

/// Main wallet manager
pub struct WalletManager {
    storage: Storage,
    mnemonic: Option<String>,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new<P: AsRef<Path>>(account_file: P) -> Self {
        WalletManager {
            storage: Storage::new(account_file),
            mnemonic: None,
        }
    }

    /// Initialize wallet with existing account file
    pub fn init(&mut self) -> Result<AccountManager> {
        self.storage.init_account_manager()
    }

    /// Create a new wallet with BIP39 mnemonic
    pub fn create_wallet(&mut self) -> Result<(AccountManager, String)> {
        if self.storage.exists() {
            return Err(anyhow!("Wallet already exists"));
        }

        let mnemonic = generate_mnemonic()?;
        self.mnemonic = Some(mnemonic.clone());

        let account_manager = create_account_manager_with_mnemonic(&mnemonic)?;
        self.storage.save_account_manager(&account_manager)?;

        Ok((account_manager, mnemonic))
    }

    /// Import wallet from mnemonic
    pub fn import_wallet(&mut self, mnemonic: &str) -> Result<AccountManager> {
        if self.storage.exists() {
            return Err(anyhow!("Wallet already exists"));
        }

        if !validate_mnemonic(mnemonic)? {
            return Err(anyhow!("Invalid mnemonic phrase"));
        }

        self.mnemonic = Some(mnemonic.to_string());
        let account_manager = create_account_manager_with_mnemonic(mnemonic)?;
        self.storage.save_account_manager(&account_manager)?;

        Ok(account_manager)
    }

    /// Add new account using mnemonic
    pub fn add_account(&mut self, mnemonic: &str) -> Result<AccountKeyPair> {
        if !self.storage.exists() {
            return Err(anyhow!("Wallet not found"));
        }

        if !validate_mnemonic(mnemonic)? {
            return Err(anyhow!("Invalid mnemonic phrase"));
        }

        let mut manager = self.storage.load_account_manager()?;
        let new_account = add_account_from_mnemonic(&mut manager, mnemonic)?;
        self.storage.save_account_manager(&manager)?;

        Ok(new_account)
    }

    /// List all accounts
    pub fn list_accounts(&self) -> Result<Vec<AccountInfo>> {
        if !self.storage.exists() {
            return Err(anyhow!("Wallet not found"));
        }

        let manager = self.storage.load_account_manager()?;
        let mut accounts = Vec::new();

        for (index, account) in manager.account_map.iter().enumerate() {
            let is_selected = account == &manager.now_selected_account;
            accounts.push(get_account_info(account, index, is_selected));
        }

        Ok(accounts)
    }

    /// Get selected account
    pub fn get_selected_account(&self) -> Result<AccountKeyPair> {
        self.storage.get_selected_account()
    }

    /// Sign transaction with selected account
    pub fn sign_transaction(&self, data: &[u8]) -> Result<[u8; 64]> {
        let selected_account = self.storage.get_selected_account()?;
        sign_transaction_data(data, &selected_account.private_key)
    }

    /// Export private key of selected account
    pub fn export_private_key(&self) -> Result<String> {
        let selected_account = self.storage.get_selected_account()?;
        Ok(selected_account.get_private_key_hex())
    }

    /// Check if wallet exists
    pub fn exists(&self) -> bool {
        self.storage.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_wallet_creation() {
        let temp_dir = TempDir::new().unwrap();
        let account_file = temp_dir.path().join("accounts.json");
        let mut wallet = WalletManager::new(&account_file);

        let (manager, mnemonic) = wallet.create_wallet().unwrap();
        assert_eq!(manager.account_map.len(), 1);
        assert!(mnemonic.split_whitespace().count() == 24);
    }

    #[test]
    fn test_wallet_import() {
        let temp_dir = TempDir::new().unwrap();
        let account_file = temp_dir.path().join("accounts.json");
        let mut wallet = WalletManager::new(&account_file);

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let manager = wallet.import_wallet(mnemonic).unwrap();
        assert_eq!(manager.account_map.len(), 1);
    }

    #[test]
    fn test_add_account() {
        let temp_dir = TempDir::new().unwrap();
        let account_file = temp_dir.path().join("accounts.json");
        let mut wallet = WalletManager::new(&account_file);

        // Create wallet first
        let (_, mnemonic) = wallet.create_wallet().unwrap();

        // Add another account
        let new_account = wallet.add_account(&mnemonic).unwrap();
        assert_eq!(new_account.public_key.0.len(), 33);

        let accounts = wallet.list_accounts().unwrap();
        assert_eq!(accounts.len(), 2);
    }
}
