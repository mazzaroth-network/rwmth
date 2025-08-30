# RMTH - Mazzaroth Wallet Manager

A Rust implementation of a wallet manager for the Mazzaroth blockchain, inspired by Geth's account management functionality.

## Features

- **BIP39 Mnemonic Support**: Generate and import wallets using BIP39 mnemonic phrases
- **secp256k1 Key Management**: Uses secp256k1 for key generation and signing (compatible with Mazzaroth)
- **Account Management**: Create, import, export, and manage multiple accounts
- **Transaction Signing**: Sign transactions for the Mazzaroth blockchain
- **Persistent Storage**: Secure local storage using JSON files (compatible with your existing format)
- **CLI Interface**: Easy-to-use command-line interface
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd rmth

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Creating a New Wallet

```bash
# Create a new wallet with BIP39 mnemonic (default name)
rmth new

# Create a new wallet with custom name
rmth new --name mywallet
```

### Managing Wallets and Accounts

```bash
# List all wallets
rmth list-wallets

# List all accounts in current wallet
rmth list

# List accounts in specific wallet
rmth --account-file ./wallets/mywallet.json list

# Show selected account
rmth selected

# Add new account using mnemonic
rmth add "your mnemonic phrase here"
```

### Importing and Exporting

```bash
# Import wallet from mnemonic phrase (default name)
rmth import "large bread source replace round mesh camera slow squirrel return swing push wrestle law ankle drive carpet survey absent afraid dove mother cluster truly"

# Import wallet with custom name
rmth import "large bread source replace round mesh camera slow squirrel return swing push wrestle law ankle drive carpet survey absent afraid dove mother cluster truly" --name mywallet

# Export private key of selected account
rmth export
```

### Transaction Signing

```bash
# Sign a transaction
rmth sign "0x1234567890abcdef..."
```

### Wallet Information

```bash
# Show wallet information
rmth info
```

## Security Features

### BIP39 Mnemonic

- **24-Word Phrases**: Uses BIP39 standard for mnemonic generation (256-bit entropy)
- **Deterministic**: Same mnemonic always generates the same keys
- **Backup Friendly**: Easy to backup and restore wallets

### Storage

- **Multi-Wallet Support**: Each wallet is stored as a separate JSON file in the `wallets/` directory
- **JSON Format**: Compatible with your existing account file format
- **Local Storage**: All data is stored locally in JSON files
- **Human Readable**: Easy to inspect and backup
- **Git Ignored**: The `wallets/` directory is automatically ignored by git for security

### Key Management

- **secp256k1**: Uses secp256k1 for key generation and signing (compatible with Mazzaroth)
- **Deterministic Addresses**: Addresses are derived from public keys using SHA-256
- **HD Wallet Ready**: Framework for hierarchical deterministic wallets

## Architecture

The wallet manager is built with a modular architecture:

```
src/
├── main.rs          # CLI interface and command handling
├── lib.rs           # Library exports
├── wallet.rs        # Main wallet manager logic
├── crypto.rs        # Cryptographic operations (BIP39, secp256k1)
├── storage.rs       # JSON file storage layer
└── types.rs         # Data structures and types
```

### Key Components

1. **WalletManager**: Orchestrates all wallet operations
2. **Storage**: Handles JSON file storage (compatible with your format)
3. **Crypto**: Manages BIP39 mnemonic and secp256k1 operations
4. **CLI**: Provides user-friendly command-line interface

## Mazzaroth Integration

This wallet is specifically designed for the Mazzaroth blockchain:

- **secp256k1 Signatures**: Compatible with Mazzaroth's signature scheme
- **Address Format**: Uses 20-byte addresses (like Ethereum)
- **Account Format**: Compatible with your existing JSON account format
- **Transaction Format**: Supports Mazzaroth transaction structure
- **MTH Token**: Designed to work with Mazzaroth's native MTH token

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

## Security Considerations

1. **Password Strength**: Use strong, unique passwords for your wallet
2. **Backup**: Regularly backup your wallet data directory
3. **Environment**: Run on secure, trusted systems
4. **Network**: Be cautious when using on public networks
5. **Updates**: Keep the wallet software updated

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Geth's account management](https://geth.ethereum.org/docs)
- Built for the [Mazzaroth blockchain](https://github.com/mazzarothnet/mazzaroth)
- Uses modern Rust cryptography libraries

## Roadmap

- [ ] Web3 integration for Mazzaroth RPC
- [ ] Hardware wallet support
- [ ] Multi-signature accounts
- [ ] Transaction history
- [ ] Network synchronization
- [ ] GUI interface
- [ ] Mobile support

## Support

For issues and questions:

1. Check the documentation
2. Search existing issues
3. Create a new issue with detailed information
4. Join the Mazzaroth community on Discord

## Disclaimer

This software is provided "as is" without warranty. Use at your own risk. Always verify transactions and addresses before sending funds.
