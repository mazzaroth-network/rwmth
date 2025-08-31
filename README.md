# 🔐 Mazzaroth Wallet Manager

A secure, modern desktop application for managing Mazzaroth blockchain accounts, built with Tauri and React.

## Features

- **🔐 BIP39 Wallet Support**: Create and import wallets using BIP39 mnemonic phrases
- **💼 Multi-Account Management**: Manage multiple accounts within a single wallet
- **✍️ Transaction Signing**: Sign transactions with your private keys
- **🔒 Secure Storage**: Encrypted private key storage with PBKDF2
- **🎨 Modern UI**: Beautiful, responsive interface built with React
- **⚡ Native Performance**: Built with Tauri for optimal performance and security

## Screenshots

![Wallet Manager Interface](screenshots/wallet-interface.png)

## Prerequisites

Before you begin, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) (for package management)

### System Dependencies

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Windows
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

## Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/mazzaroth-network/rwmth.git
   cd rmth
   ```

2. **Install dependencies**
   ```bash
   bun install
   ```

3. **Run the development server**
   ```bash
   bun run tauri dev
   ```

## Building for Production

### Development Build
```bash
bun run tauri dev
```

### Production Build
```bash
bun run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/`.

## Usage

### Creating a New Wallet

1. Launch the application
2. Enter a wallet name in the "Wallet name" field
3. Click "Create Wallet"
4. **IMPORTANT**: Save the generated mnemonic phrase securely - this is your only backup!

### Importing an Existing Wallet

1. Enter a wallet name
2. Enter your 24-word BIP39 mnemonic phrase
3. Click "Import Wallet"

### Managing Accounts

1. Load a wallet by clicking on it in the wallet list
2. Add new accounts using the mnemonic phrase
3. Select accounts by clicking on them
4. View account details and public keys

### Signing Transactions

1. Load a wallet and select an account
2. Enter transaction data in hex format
3. Click "Sign Transaction"
4. Copy the generated signature

## Security Features

- **BIP39 Compliance**: Uses industry-standard BIP39 for mnemonic generation
- **Encrypted Storage**: Private keys are encrypted using PBKDF2
- **Secure Derivation**: HD wallet derivation following BIP44 standards
- **Memory Safety**: Built with Rust for memory safety and performance

## Architecture

### Frontend (React + TypeScript)
- Modern React with hooks and functional components
- TypeScript for type safety
- Responsive design with CSS Grid and Flexbox
- Real-time updates and error handling

### Backend (Rust + Tauri)
- Rust for performance and security
- Tauri for native desktop capabilities
- BIP39/BIP44 wallet implementation
- Encrypted storage with PBKDF2

### Key Components

```
src/
├── App.tsx              # Main application component
├── App.css              # Application styles
└── main.tsx             # Application entry point

src-tauri/src/
├── lib.rs               # Tauri commands and app setup
├── crypto.rs            # Cryptographic operations
├── storage.rs           # File system storage
├── types.rs             # Data structures
└── wallet.rs            # Wallet management logic
```

## Development

### Project Structure
```
rmth/
├── src/                 # Frontend React code
├── src-tauri/          # Backend Rust code
├── cli/                # CLI version (original)
├── public/             # Static assets
└── dist/               # Built frontend
```

### Available Scripts

- `bun run dev` - Start development server
- `bun run build` - Build frontend for production
- `bun run tauri dev` - Run Tauri development build
- `bun run tauri build` - Build Tauri application
- `bun run preview` - Preview production build

### Adding New Features

1. **Backend (Rust)**: Add new Tauri commands in `src-tauri/src/lib.rs`
2. **Frontend (React)**: Add new components and update `src/App.tsx`
3. **Types**: Update interfaces in both Rust and TypeScript

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing desktop framework
- [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) - For the mnemonic standard
- [React](https://react.dev/) - For the frontend framework
- [Rust](https://rust-lang.org/) - For the backend language

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/rmth/issues) page
2. Create a new issue with detailed information
3. Join our [Discord](https://discord.gg/your-server) for community support

## Roadmap

- [ ] Hardware wallet support
- [ ] Multi-language support
- [ ] Advanced transaction builder
- [ ] Network integration
- [ ] Mobile app version
- [ ] Plugin system for custom features

---

**⚠️ Security Warning**: This is experimental software. Always test with small amounts and never use for large transactions without thorough testing.
