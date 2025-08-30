use clap::{Parser, Subcommand};
use rmth::wallet::WalletManager;
use tracing::info;

#[derive(Parser)]
#[command(name = "rmth")]
#[command(
    about = "Mazzaroth BIP39 Wallet Manager - A Rust implementation for managing Mazzaroth blockchain accounts"
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to account file
    #[arg(short, long, default_value = "./wallets/default.json")]
    account_file: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet with BIP39 mnemonic
    New {
        /// Wallet name (optional, defaults to "default")
        #[arg(short, long, default_value = "default")]
        name: String,
    },

    /// Import wallet from mnemonic phrase
    Import {
        /// BIP39 mnemonic phrase (24 words)
        mnemonic: String,
        
        /// Wallet name (optional, defaults to "default")
        #[arg(short, long, default_value = "default")]
        name: String,
    },

    /// List all accounts
    List,

    /// List all wallets
    ListWallets,

    /// Add new account using mnemonic
    Add {
        /// BIP39 mnemonic phrase
        mnemonic: String,
    },

    /// Show selected account
    Selected,

    /// Export private key of selected account
    Export,

    /// Sign transaction data
    Sign {
        /// Transaction data (hex format)
        data: String,
    },

    /// Show wallet information
    Info,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    // Determine account file path based on wallet name
    let account_file = match &cli.command {
        Commands::New { name } => format!("./wallets/{}.json", name),
        Commands::Import { name, .. } => format!("./wallets/{}.json", name),
        _ => cli.account_file.clone(),
    };
    
    let mut wallet = WalletManager::new(&account_file);

    match cli.command {
        Commands::New { name } => {
            info!("Creating new wallet: {}", name);
            let (manager, mnemonic) = wallet.create_wallet()?;
            println!("✅ Wallet '{}' created successfully!", name);
            println!("📝 BIP39 Mnemonic (SAVE THIS SECURELY):");
            println!("   {}", mnemonic);
            println!(
                "🔑 First account address: {}",
                manager.now_selected_account.get_address()
            );
        }

        Commands::Import { mnemonic, name } => {
            info!("Importing wallet from mnemonic: {}", name);
            let manager = wallet.import_wallet(&mnemonic)?;
            println!("✅ Wallet '{}' imported successfully!", name);
            println!(
                "🔑 First account address: {}",
                manager.now_selected_account.get_address()
            );
        }

        Commands::List => {
            info!("Listing accounts");
            let accounts = wallet.list_accounts()?;
            if accounts.is_empty() {
                println!("No accounts found.");
            } else {
                println!("Accounts in wallet:");
                for account in accounts {
                    let status = if account.is_selected {
                        " (selected)"
                    } else {
                        ""
                    };
                    println!(
                        "  {}. {} - {}{}",
                        account.index + 1,
                        account.address,
                        account.public_key,
                        status
                    );
                }
            }
        }

        Commands::ListWallets => {
            info!("Listing all wallets");
            let wallets_dir = std::path::Path::new("./wallets");
            if !wallets_dir.exists() {
                println!("No wallets directory found.");
                return Ok(());
            }
            
            let mut wallets = Vec::new();
            if let Ok(entries) = std::fs::read_dir(wallets_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(extension) = entry.path().extension() {
                            if extension == "json" {
                                if let Some(file_name) = entry.path().file_stem() {
                                    if let Some(name) = file_name.to_str() {
                                        wallets.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if wallets.is_empty() {
                println!("No wallets found.");
            } else {
                println!("Available wallets:");
                for (i, wallet_name) in wallets.iter().enumerate() {
                    println!("  {}. {}", i + 1, wallet_name);
                }
            }
        }

        Commands::Add { mnemonic } => {
            info!("Adding new account");
            let account = wallet.add_account(&mnemonic)?;
            println!("✅ New account added successfully!");
            println!("🔑 Address: {}", account.get_address());
        }

        Commands::Selected => {
            info!("Showing selected account");
            let account = wallet.get_selected_account()?;
            println!("Selected Account:");
            println!("  Address: {}", account.get_address());
            println!("  Public Key: {}", account.get_public_key_hex());
        }

        Commands::Export => {
            info!("Exporting private key");
            let private_key = wallet.export_private_key()?;
            println!("Private Key: {}", private_key);
        }

        Commands::Sign { data } => {
            info!("Signing transaction");
            let data_bytes =
                hex::decode(data.trim_start_matches("0x")).map_err(|_| "Invalid hex data")?;
            let signature = wallet.sign_transaction(&data_bytes)?;
            println!("Signature: 0x{}", hex::encode(signature));
        }

        Commands::Info => {
            info!("Showing wallet information");
            if wallet.exists() {
                println!("Wallet Information:");
                println!("  Account File: {}", cli.account_file);
                println!("  Status: Initialized");

                let accounts = wallet.list_accounts()?;
                println!("  Total Accounts: {}", accounts.len());

                let selected = wallet.get_selected_account()?;
                println!("  Selected Account: {}", selected.get_address());
            } else {
                println!("Wallet not found. Use 'new' to create a wallet or 'import' to import from mnemonic.");
            }
        }
    }

    Ok(())
}
