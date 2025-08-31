pub mod crypto;
pub mod storage;
pub mod types;
pub mod wallet;

use crate::types::*;
use crate::wallet::WalletManager;
use std::sync::Mutex;
use tauri::State;

// Global wallet manager state
type WalletState = Mutex<WalletManager>;

#[tauri::command]
async fn create_wallet(
    wallet_name: String,
    state: State<'_, WalletState>,
) -> Result<CreateWalletResponse, String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .create_wallet(&wallet_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_wallet(
    wallet_name: String,
    mnemonic: String,
    state: State<'_, WalletState>,
) -> Result<ImportWalletResponse, String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .import_wallet(&wallet_name, &mnemonic)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_wallet(wallet_name: String, state: State<'_, WalletState>) -> Result<bool, String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .load_wallet(&wallet_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_wallets(state: State<'_, WalletState>) -> Result<Vec<String>, String> {
    let wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager.list_wallets().map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_accounts(state: State<'_, WalletState>) -> Result<Vec<AccountInfo>, String> {
    let wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager.list_accounts().map_err(|e| e.to_string())
}

#[tauri::command]
async fn select_account(index: usize, state: State<'_, WalletState>) -> Result<(), String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .select_account(index)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_selected_account(
    state: State<'_, WalletState>,
) -> Result<Option<AccountInfo>, String> {
    let wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .get_selected_account()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_account(
    mnemonic: String,
    state: State<'_, WalletState>,
) -> Result<AccountInfo, String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .add_account(&mnemonic)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_account(state: State<'_, WalletState>) -> Result<AccountInfo, String> {
    let mut wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager.create_account().map_err(|e| e.to_string())
}

#[tauri::command]
async fn sign_transaction(
    data: String,
    state: State<'_, WalletState>,
) -> Result<SignTransactionResponse, String> {
    let wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager
        .sign_transaction(&data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_wallet_info(state: State<'_, WalletState>) -> Result<Option<WalletInfo>, String> {
    let wallet_manager = state.lock().map_err(|_| "Failed to lock wallet manager")?;
    wallet_manager.get_wallet_info().map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Mazzaroth Wallet Manager!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(WalletManager::new()))
        .invoke_handler(tauri::generate_handler![
            greet,
            create_wallet,
            import_wallet,
            load_wallet,
            list_wallets,
            list_accounts,
            select_account,
            get_selected_account,
            add_account,
            create_account,
            sign_transaction,
            get_wallet_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
