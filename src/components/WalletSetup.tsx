import React, { useState } from 'react';

interface WalletSetupProps {
  wallets: string[];
  currentWallet: string;
  newWalletName: string;
  setNewWalletName: (name: string) => void;
  mnemonic: string;
  setMnemonic: (mnemonic: string) => void;
  loading: boolean;
  createWallet: () => void;
  importWallet: () => void;
  loadWallet: (walletName: string) => void;
}

const WalletSetup: React.FC<WalletSetupProps> = ({
  wallets,
  currentWallet,
  newWalletName,
  setNewWalletName,
  mnemonic,
  setMnemonic,
  loading,
  createWallet,
  importWallet,
  loadWallet,
}) => {
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showImportForm, setShowImportForm] = useState(false);
  return (
    <div className="wallet-setup">
      <div className="wallet-setup-header">
        <h2>Wallet Management</h2>
        <div className="wallet-actions">
          <button 
            className="create-wallet-btn"
            onClick={() => setShowCreateForm(true)}
            disabled={loading}
          >
            Create Wallet
          </button>
          <button 
            className="import-wallet-btn"
            onClick={() => setShowImportForm(true)}
            disabled={loading}
          >
            Import Wallet
          </button>
        </div>
      </div>

      {wallets.length > 0 && (
        <div className="wallet-list">
          <h3>Available Wallets</h3>
          <div className="wallet-grid">
            {wallets.map((wallet) => (
              <div
                key={wallet}
                className={`wallet-item ${currentWallet === wallet ? "selected" : ""}`}
                onClick={() => loadWallet(wallet)}
              >
                <span>📂 {wallet}</span>
                {currentWallet === wallet && <span className="badge">Active</span>}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Create Wallet Form */}
      {showCreateForm && (
        <div className="wallet-form-overlay">
          <div className="wallet-form">
            <div className="wallet-form-header">
              <h3>Create New Wallet</h3>
              <button 
                className="close-btn"
                onClick={() => setShowCreateForm(false)}
              >
                ✕
              </button>
            </div>
            
            <div className="form-section">
              <label htmlFor="walletName">Wallet Name</label>
              <input
                id="walletName"
                type="text"
                placeholder="Enter wallet name"
                value={newWalletName}
                onChange={(e) => setNewWalletName(e.target.value)}
                disabled={loading}
              />
            </div>

            <div className="form-actions">
              <button
                onClick={() => setShowCreateForm(false)}
                className="cancel-btn"
                disabled={loading}
              >
                Cancel
              </button>
              <button
                onClick={() => {
                  createWallet();
                  setShowCreateForm(false);
                }}
                disabled={loading || !newWalletName.trim()}
                className="create-btn"
              >
                {loading ? "Creating..." : "Create Wallet"}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Import Wallet Form */}
      {showImportForm && (
        <div className="wallet-form-overlay">
          <div className="wallet-form">
            <div className="wallet-form-header">
              <h3>Import Existing Wallet</h3>
              <button 
                className="close-btn"
                onClick={() => setShowImportForm(false)}
              >
                ✕
              </button>
            </div>
            
            <div className="form-section">
              <label htmlFor="importWalletName">Wallet Name</label>
              <input
                id="importWalletName"
                type="text"
                placeholder="Enter wallet name"
                value={newWalletName}
                onChange={(e) => setNewWalletName(e.target.value)}
                disabled={loading}
              />
            </div>

            <div className="form-section">
              <label htmlFor="importMnemonic">Mnemonic Phrase</label>
              <textarea
                id="importMnemonic"
                placeholder="Enter 12, 15, 18, 21, or 24 word mnemonic phrase"
                value={mnemonic}
                onChange={(e) => setMnemonic(e.target.value)}
                disabled={loading}
                rows={4}
              />
              <p className="input-help">
                Enter the mnemonic phrase for the wallet you want to import.
              </p>
            </div>

            <div className="form-actions">
              <button
                onClick={() => setShowImportForm(false)}
                className="cancel-btn"
                disabled={loading}
              >
                Cancel
              </button>
              <button
                onClick={() => {
                  importWallet();
                  setShowImportForm(false);
                }}
                disabled={loading || !newWalletName.trim() || !mnemonic.trim()}
                className="import-btn"
              >
                {loading ? "Importing..." : "Import Wallet"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default WalletSetup;
