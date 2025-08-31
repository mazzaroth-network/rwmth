use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub public_key: String,
    pub private_key_encrypted: Vec<u8>,
    pub salt: Vec<u8>,
    pub is_selected: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(
        address: String,
        public_key: String,
        private_key_encrypted: Vec<u8>,
        salt: Vec<u8>,
    ) -> Self {
        Self {
            address,
            public_key,
            private_key_encrypted,
            salt,
            is_selected: false,
            created_at: Utc::now(),
            last_used: None,
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_public_key_hex(&self) -> &str {
        &self.public_key
    }

    pub fn mark_as_selected(&mut self) {
        self.is_selected = true;
        self.last_used = Some(Utc::now());
    }

    pub fn mark_as_unselected(&mut self) {
        self.is_selected = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub accounts: Vec<Account>,
    pub selected_account_index: Option<usize>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub version: String,
}

impl WalletData {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            selected_account_index: None,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            version: "1.0.0".to_string(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
        self.last_modified = Utc::now();
    }

    pub fn select_account(&mut self, index: usize) -> Result<(), String> {
        if index >= self.accounts.len() {
            return Err("Account index out of bounds".to_string());
        }

        // Unselect all accounts
        for account in &mut self.accounts {
            account.mark_as_unselected();
        }

        // Select the specified account
        self.accounts[index].mark_as_selected();
        self.selected_account_index = Some(index);
        self.last_modified = Utc::now();
        Ok(())
    }

    pub fn get_selected_account(&self) -> Option<&Account> {
        self.selected_account_index
            .and_then(|index| self.accounts.get(index))
    }

    pub fn get_selected_account_mut(&mut self) -> Option<&mut Account> {
        self.selected_account_index
            .and_then(|index| self.accounts.get_mut(index))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub total_accounts: usize,
    pub selected_account: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletResponse {
    pub success: bool,
    pub mnemonic: String,
    pub address: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportWalletResponse {
    pub success: bool,
    pub address: String,
    pub public_key: String,
    pub total_accounts: usize,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub address: String,
    pub public_key: String,
    pub is_selected: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignTransactionRequest {
    pub data: String, // Hex string
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignTransactionResponse {
    pub success: bool,
    pub signature: String, // Hex string
    pub message: String,
}
