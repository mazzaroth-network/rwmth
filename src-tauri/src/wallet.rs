use anyhow::Result;
use rand::Rng;
use hex;
use crate::crypto::CryptoManager;
use crate::storage::StorageManager;
use crate::types::{Account, WalletData, CreateWalletResponse, ImportWalletResponse, AccountInfo, SignTransactionResponse};

const COIN_TYPE: u32 = 55555;

pub struct WalletManager {
    storage: StorageManager,
    current_wallet: Option<String>,
    wallet_data: Option<WalletData>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            storage: StorageManager::new(),
            current_wallet: None,
            wallet_data: None,
        }
    }

    pub fn create_wallet(&mut self, wallet_name: &str) -> Result<CreateWalletResponse> {
        // Generate new mnemonic
        let mnemonic = CryptoManager::generate_mnemonic()?;
        
        // Create wallet data
        let mut wallet_data = WalletData::new();
        
        // Derive first account
        let private_key = CryptoManager::derive_private_key(&mnemonic, "m/44'/{COIN_TYPE}'/0'/0/0")?;
        let public_key = CryptoManager::get_public_key(&private_key);
        let address = CryptoManager::generate_address(&public_key);
        
        // Encrypt private key
        let salt: [u8; 32] = rand::thread_rng().gen();
        let encrypted_private_key = CryptoManager::encrypt_data(&private_key.secret_bytes(), "default_password", &salt)?;
        
        // Create account
        let account = Account::new(
            address.clone(),
            hex::encode(public_key.serialize_uncompressed()),
            encrypted_private_key,
            salt.to_vec(),
        );
        
        wallet_data.add_account(account);
        wallet_data.select_account(0).map_err(|e| anyhow::anyhow!(e))?;
        
        // Save wallet
        self.storage.save_wallet(wallet_name, &wallet_data)?;
        
        // Update current state
        self.current_wallet = Some(wallet_name.to_string());
        self.wallet_data = Some(wallet_data);
        
        Ok(CreateWalletResponse {
            success: true,
            mnemonic,
            address,
            public_key: hex::encode(public_key.serialize_uncompressed()),
            message: "Wallet created successfully".to_string(),
        })
    }

    pub fn import_wallet(&mut self, wallet_name: &str, mnemonic: &str) -> Result<ImportWalletResponse> {
        // Derive first account from mnemonic
        let private_key = CryptoManager::derive_private_key(mnemonic, "m/44'/{COIN_TYPE}'/0'/0/0")?;
        let public_key = CryptoManager::get_public_key(&private_key);
        let address = CryptoManager::generate_address(&public_key);
        
        // Encrypt private key
        let salt: [u8; 32] = rand::thread_rng().gen();
        let encrypted_private_key = CryptoManager::encrypt_data(&private_key.secret_bytes(), "default_password", &salt)?;
        
        // Create wallet data
        let mut wallet_data = WalletData::new();
        let account = Account::new(
            address.clone(),
            hex::encode(public_key.serialize_uncompressed()),
            encrypted_private_key,
            salt.to_vec(),
        );
        
        wallet_data.add_account(account);
        wallet_data.select_account(0).map_err(|e| anyhow::anyhow!(e))?;
        
        // Save wallet
        self.storage.save_wallet(wallet_name, &wallet_data)?;
        
        // Update current state
        self.current_wallet = Some(wallet_name.to_string());
        self.wallet_data = Some(wallet_data);
        
        Ok(ImportWalletResponse {
            success: true,
            address,
            public_key: hex::encode(public_key.serialize_uncompressed()),
            total_accounts: 1,
            message: "Wallet imported successfully".to_string(),
        })
    }

    pub fn load_wallet(&mut self, wallet_name: &str) -> Result<bool> {
        if let Some(wallet_data) = self.storage.load_wallet(wallet_name)? {
            self.current_wallet = Some(wallet_name.to_string());
            self.wallet_data = Some(wallet_data);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn add_account(&mut self, mnemonic: &str) -> Result<AccountInfo> {
        let wallet_data = self.wallet_data.as_mut()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        // Derive new account
        let account_index = wallet_data.accounts.len();
        let path = format!("m/44'/{COIN_TYPE}'/0'/0/{}", account_index);
        let private_key = CryptoManager::derive_private_key(mnemonic, &path)?;
        let public_key = CryptoManager::get_public_key(&private_key);
        let address = CryptoManager::generate_address(&public_key);
        
        // Encrypt private key
        let salt: [u8; 32] = rand::thread_rng().gen();
        let encrypted_private_key = CryptoManager::encrypt_data(&private_key.secret_bytes(), "default_password", &salt)?;
        
        // Create account
        let account = Account::new(
            address.clone(),
            hex::encode(public_key.serialize_uncompressed()),
            encrypted_private_key,
            salt.to_vec(),
        );
        
        wallet_data.add_account(account);
        
        // Save wallet
        if let Some(wallet_name) = &self.current_wallet {
            self.storage.save_wallet(wallet_name, wallet_data)?;
        }
        
        Ok(AccountInfo {
            address,
            public_key: hex::encode(public_key.serialize_uncompressed()),
            is_selected: false,
            created_at: chrono::Utc::now(),
        })
    }

    pub fn create_account(&mut self) -> Result<AccountInfo> {
        let wallet_data = self.wallet_data.as_mut()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        // For now, we'll use the same logic as add_account but without requiring mnemonic
        // In a real implementation, we'd derive from the wallet's master seed
        let account_index = wallet_data.accounts.len();
        let _path = format!("m/44'/{COIN_TYPE}'/0'/0/{}", account_index);
        
        // Generate a new private key for this account
        let private_key = secp256k1::SecretKey::new(&mut rand::thread_rng());
        let public_key = CryptoManager::get_public_key(&private_key);
        let address = CryptoManager::generate_address(&public_key);
        
        // Encrypt private key
        let salt: [u8; 32] = rand::thread_rng().gen();
        let encrypted_private_key = CryptoManager::encrypt_data(&private_key.secret_bytes(), "default_password", &salt)?;
        
        // Create account
        let account = Account::new(
            address.clone(),
            hex::encode(public_key.serialize_uncompressed()),
            encrypted_private_key,
            salt.to_vec(),
        );
        
        wallet_data.add_account(account);
        
        // Save wallet
        if let Some(wallet_name) = &self.current_wallet {
            self.storage.save_wallet(wallet_name, wallet_data)?;
        }
        
        Ok(AccountInfo {
            address,
            public_key: hex::encode(public_key.serialize_uncompressed()),
            is_selected: false,
            created_at: chrono::Utc::now(),
        })
    }

    pub fn list_accounts(&self) -> Result<Vec<AccountInfo>> {
        let wallet_data = self.wallet_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        let mut accounts = Vec::new();
        for account in &wallet_data.accounts {
            accounts.push(AccountInfo {
                address: account.address.clone(),
                public_key: account.public_key.clone(),
                is_selected: account.is_selected,
                created_at: account.created_at,
            });
        }
        
        Ok(accounts)
    }

    pub fn select_account(&mut self, index: usize) -> Result<()> {
        let wallet_data = self.wallet_data.as_mut()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        wallet_data.select_account(index).map_err(|e| anyhow::anyhow!(e))?;
        
        // Save wallet
        if let Some(wallet_name) = &self.current_wallet {
            self.storage.save_wallet(wallet_name, wallet_data)?;
        }
        
        Ok(())
    }

    pub fn get_selected_account(&self) -> Result<Option<AccountInfo>> {
        let wallet_data = self.wallet_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        if let Some(account) = wallet_data.get_selected_account() {
            Ok(Some(AccountInfo {
                address: account.address.clone(),
                public_key: account.public_key.clone(),
                is_selected: account.is_selected,
                created_at: account.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn sign_transaction(&self, data: &str) -> Result<SignTransactionResponse> {
        let wallet_data = self.wallet_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        let selected_account = wallet_data.get_selected_account()
            .ok_or_else(|| anyhow::anyhow!("No account selected"))?;
        
        // Decrypt private key
        let private_key_bytes = CryptoManager::decrypt_data(
            &selected_account.private_key_encrypted,
            "default_password",
            &selected_account.salt,
        )?;
        
        let private_key = secp256k1::SecretKey::from_slice(&private_key_bytes)?;
        
        // Decode hex data
        let data_bytes = hex::decode(data.trim_start_matches("0x"))?;
        
        // Sign data
        let signature = CryptoManager::sign_data(&private_key, &data_bytes)?;
        
        Ok(SignTransactionResponse {
            success: true,
            signature: hex::encode(signature),
            message: "Transaction signed successfully".to_string(),
        })
    }

    pub fn list_wallets(&self) -> Result<Vec<String>> {
        self.storage.list_wallets()
    }

    pub fn get_wallet_info(&self) -> Result<Option<crate::types::WalletInfo>> {
        let wallet_data = self.wallet_data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No wallet loaded"))?;
        
        Ok(Some(crate::types::WalletInfo {
            total_accounts: wallet_data.accounts.len(),
            selected_account: wallet_data.get_selected_account().map(|a| a.address.clone()),
            created_at: wallet_data.created_at,
            last_modified: wallet_data.last_modified,
        }))
    }
}
