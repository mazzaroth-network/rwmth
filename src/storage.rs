use crate::types::{AccountKeyPair, AccountManager};
use anyhow::{anyhow, Result};
use serde_json;
use std::fs;
use std::path::Path;

/// Storage manager for account data
pub struct Storage {
    file_path: String,
}

impl Storage {
    /// Create a new storage instance
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Storage {
            file_path: file_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Load account manager from file
    pub fn load_account_manager(&self) -> Result<AccountManager> {
        if !Path::new(&self.file_path).exists() {
            return Err(anyhow!("Account file not found: {}", self.file_path));
        }

        let content = fs::read_to_string(&self.file_path)?;
        let account_manager: AccountManager = serde_json::from_str(&content)?;
        Ok(account_manager)
    }

    /// Save account manager to file
    pub fn save_account_manager(&self, account_manager: &AccountManager) -> Result<()> {
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(&self.file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(account_manager)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    /// Initialize account manager (create if doesn't exist)
    pub fn init_account_manager(&self) -> Result<AccountManager> {
        if Path::new(&self.file_path).exists() {
            self.load_account_manager()
        } else {
            // Create default account manager
            let account_manager = AccountManager::default();
            self.save_account_manager(&account_manager)?;
            Ok(account_manager)
        }
    }

    /// Add account to manager and save
    pub fn add_account(&self, account: AccountKeyPair) -> Result<AccountManager> {
        let mut manager = self.load_account_manager()?;
        manager.account_map.push(account.clone());
        manager.now_selected_account = account;
        self.save_account_manager(&manager)?;
        Ok(manager)
    }

    /// Remove account by index
    pub fn remove_account(&self, index: usize) -> Result<AccountManager> {
        let mut manager = self.load_account_manager()?;

        if index >= manager.account_map.len() {
            return Err(anyhow!("Account index out of bounds"));
        }

        manager.account_map.remove(index);

        // Update selected account if needed
        if manager.account_map.is_empty() {
            return Err(anyhow!("Cannot remove the last account"));
        }

        if index == 0 || manager.now_selected_account == manager.account_map[index - 1] {
            manager.now_selected_account = manager.account_map[0].clone();
        }

        self.save_account_manager(&manager)?;
        Ok(manager)
    }

    /// Set selected account by index
    pub fn set_selected_account(&self, index: usize) -> Result<AccountManager> {
        let mut manager = self.load_account_manager()?;

        if index >= manager.account_map.len() {
            return Err(anyhow!("Account index out of bounds"));
        }

        manager.now_selected_account = manager.account_map[index].clone();
        self.save_account_manager(&manager)?;
        Ok(manager)
    }

    /// Get account by index
    pub fn get_account(&self, index: usize) -> Result<AccountKeyPair> {
        let manager = self.load_account_manager()?;

        if index >= manager.account_map.len() {
            return Err(anyhow!("Account index out of bounds"));
        }

        Ok(manager.account_map[index].clone())
    }

    /// Get selected account
    pub fn get_selected_account(&self) -> Result<AccountKeyPair> {
        let manager = self.load_account_manager()?;
        Ok(manager.now_selected_account.clone())
    }

    /// List all accounts
    pub fn list_accounts(&self) -> Result<Vec<AccountKeyPair>> {
        let manager = self.load_account_manager()?;
        Ok(manager.account_map.clone())
    }

    /// Get account count
    pub fn get_account_count(&self) -> Result<usize> {
        let manager = self.load_account_manager()?;
        Ok(manager.account_map.len())
    }

    /// Check if account file exists
    pub fn exists(&self) -> bool {
        Path::new(&self.file_path).exists()
    }

    /// Get file path
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    /// Backup account file
    pub fn backup(&self, backup_path: &str) -> Result<()> {
        if !self.exists() {
            return Err(anyhow!("Account file does not exist"));
        }

        fs::copy(&self.file_path, backup_path)?;
        Ok(())
    }

    /// Restore account file from backup
    pub fn restore(&self, backup_path: &str) -> Result<()> {
        if !Path::new(backup_path).exists() {
            return Err(anyhow!("Backup file does not exist"));
        }

        // Validate backup file format
        let content = fs::read_to_string(backup_path)?;
        let _: AccountManager = serde_json::from_str(&content)?;

        fs::copy(backup_path, &self.file_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_keypair;
    use tempfile::TempDir;

    #[test]
    fn test_storage_operations() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("accounts.json");
        let storage = Storage::new(&file_path);

        // Test initialization
        let manager = storage.init_account_manager().unwrap();
        assert_eq!(manager.account_map.len(), 1);

        // Test adding account
        let new_account = generate_keypair().unwrap();
        let updated_manager = storage.add_account(new_account.clone()).unwrap();
        assert_eq!(updated_manager.account_map.len(), 2);

        // Test getting account
        let retrieved_account = storage.get_account(1).unwrap();
        assert_eq!(retrieved_account.public_key.0, new_account.public_key.0);

        // Test setting selected account
        let manager = storage.set_selected_account(1).unwrap();
        assert_eq!(
            manager.now_selected_account.public_key.0,
            new_account.public_key.0
        );
    }

    #[test]
    fn test_backup_restore() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("accounts.json");
        let backup_path = temp_dir.path().join("accounts_backup.json");

        let storage = Storage::new(&file_path);

        // Initialize and backup
        storage.init_account_manager().unwrap();
        storage.backup(backup_path.to_str().unwrap()).unwrap();

        // Modify original
        let new_account = generate_keypair().unwrap();
        storage.add_account(new_account).unwrap();

        // Restore from backup
        storage.restore(backup_path.to_str().unwrap()).unwrap();

        // Verify restored
        let manager = storage.load_account_manager().unwrap();
        assert_eq!(manager.account_map.len(), 1);
    }
}
