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

            println!("\n🎉 Wallet Creation Successful!");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📁 Wallet Name: {}", name);
            println!("📂 Storage Path: ./wallets/{}.json", name);
            println!("🔐 Security Level: BIP39 (256-bit entropy)");
            println!("🌐 Blockchain: Mazzaroth");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n⚠️  CRITICAL SECURITY INFORMATION ⚠️");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📝 BIP39 Recovery Phrase (24 words):");
            println!("   {}", mnemonic);
            println!("\n🔒 SECURITY REQUIREMENTS:");
            println!("   • Write down this phrase on paper");
            println!("   • Store in a secure, fireproof location");
            println!("   • Never share with anyone");
            println!("   • This is your only backup method");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n💼 Account Information:");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🔑 Primary Account Address:");
            println!("   {}", manager.now_selected_account.get_address());
            println!("📊 Public Key:");
            println!("   {}", manager.now_selected_account.public_key.to_hex());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n🚀 Next Steps:");
            println!("   • Backup your recovery phrase securely");
            println!("   • Test with small amounts first");
            println!("   • Use 'rmth list' to view accounts");
            println!("   • Use 'rmth info' for wallet details");
            println!("\n✅ Wallet '{}' is ready for use!", name);
        }

        Commands::Import { mnemonic, name } => {
            info!("Importing wallet from mnemonic: {}", name);
            let manager = wallet.import_wallet(&mnemonic)?;

            println!("\n🎉 Wallet Import Successful!");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📁 Wallet Name: {}", name);
            println!("📂 Storage Path: ./wallets/{}.json", name);
            println!("🔐 Security Level: BIP39 (256-bit entropy)");
            println!("🌐 Blockchain: Mazzaroth");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n💼 Account Information:");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("🔑 Primary Account Address:");
            println!("   {}", manager.now_selected_account.get_address());
            println!("📊 Public Key:");
            println!("   {}", manager.now_selected_account.public_key.to_hex());
            println!("📈 Total Accounts: {}", manager.account_map.len());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n🚀 Next Steps:");
            println!("   • Verify your account addresses");
            println!("   • Use 'rmth list' to view all accounts");
            println!("   • Use 'rmth info' for wallet details");
            println!("\n✅ Wallet '{}' imported and ready for use!", name);
        }

        Commands::List => {
            info!("Listing accounts");
            let accounts = wallet.list_accounts()?;
            if accounts.is_empty() {
                println!("\n💼 No accounts found in this wallet");
                println!("💡 Add accounts with: rmth add \"your mnemonic phrase\"");
            } else {
                println!("\n💼 Accounts in Wallet:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                let total_accounts = accounts.len();
                for (i, account) in accounts.iter().enumerate() {
                    let status = if account.is_selected {
                        " 🔵 SELECTED"
                    } else {
                        ""
                    };
                    println!("  {}. 🔑 {}", i + 1, account.address);
                    println!("     📊 Public Key: {}", account.public_key);
                    println!(
                        "     📍 Status: {}{}",
                        if account.is_selected {
                            "Active"
                        } else {
                            "Inactive"
                        },
                        status
                    );
                    if i < total_accounts - 1 {
                        println!("     ──────────────────────────────────────────────");
                    }
                }
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("💡 Use 'rmth selected' to view current account details");
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
                println!("\n📁 No wallets found in ./wallets/ directory");
                println!("💡 Create your first wallet with: rmth new");
            } else {
                println!("\n📁 Available Wallets:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                for (i, wallet_name) in wallets.iter().enumerate() {
                    println!("  {}. 📂 {} ({}.json)", i + 1, wallet_name, wallet_name);
                }
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("💡 Use 'rmth --account-file ./wallets/[name].json [command]' to work with specific wallets");
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
