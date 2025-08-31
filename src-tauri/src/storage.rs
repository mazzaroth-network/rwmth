use std::fs;
use std::path::Path;
use serde_json;
use anyhow::Result;
use crate::types::WalletData;

pub struct StorageManager {
    data_dir: String,
}

impl StorageManager {
    pub fn new() -> Self {
        let data_dir = Self::get_data_dir();
        Self { data_dir }
    }

    fn get_data_dir() -> String {
        // Use Tauri's app data directory
        let app_name = "rwmth";
        let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let data_dir = home_dir.join(".config").join(app_name);
        
        // Create directory if it doesn't exist
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).ok();
        }
        
        data_dir.to_string_lossy().to_string()
    }

    pub fn save_wallet(&self, wallet_name: &str, wallet_data: &WalletData) -> Result<()> {
        let file_path = Path::new(&self.data_dir).join(format!("{}.json", wallet_name));
        let json = serde_json::to_string_pretty(wallet_data)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn load_wallet(&self, wallet_name: &str) -> Result<Option<WalletData>> {
        let file_path = Path::new(&self.data_dir).join(format!("{}.json", wallet_name));
        
        if !file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(file_path)?;
        let wallet_data: WalletData = serde_json::from_str(&content)?;
        Ok(Some(wallet_data))
    }

    pub fn wallet_exists(&self, wallet_name: &str) -> bool {
        let file_path = Path::new(&self.data_dir).join(format!("{}.json", wallet_name));
        file_path.exists()
    }

    pub fn list_wallets(&self) -> Result<Vec<String>> {
        let mut wallets = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.data_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "json" {
                            if let Some(file_stem) = entry.path().file_stem() {
                                if let Some(name) = file_stem.to_str() {
                                    wallets.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(wallets)
    }

    pub fn delete_wallet(&self, wallet_name: &str) -> Result<()> {
        let file_path = Path::new(&self.data_dir).join(format!("{}.json", wallet_name));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    pub fn get_wallet_path(&self, wallet_name: &str) -> String {
        Path::new(&self.data_dir).join(format!("{}.json", wallet_name)).to_string_lossy().to_string()
    }
}
