import React, { useState } from 'react';
import WalletSetup from './WalletSetup';
import WalletOverview from './WalletOverview';

interface AccountsPageProps {
  currentWallet: string;
  wallets: string[];
  newWalletName: string;
  setNewWalletName: (name: string) => void;
  mnemonic: string;
  setMnemonic: (mnemonic: string) => void;
  loading: boolean;
  createWallet: () => void;
  importWallet: () => void;
  loadWallet: (walletName: string) => void;
  returnToWalletSelection: () => void;
  createAccount: () => void;
  addAccount: () => void;
  selectAccount: (index: number) => void;
  accounts: any[];
}

const AccountsPage: React.FC<AccountsPageProps> = ({
  currentWallet,
  wallets,
  newWalletName,
  setNewWalletName,
  mnemonic,
  setMnemonic,
  loading,
  createWallet,
  importWallet,
  loadWallet,
  returnToWalletSelection,
  createAccount,
  addAccount,
  selectAccount,
  accounts,
}) => {
  const [showImportForm, setShowImportForm] = useState(false);
  return (
    <div className="accounts-page">
      <div className="page-header">
        <h2>Accounts</h2>
      </div>

      {!currentWallet ? (
        <WalletSetup
          wallets={wallets}
          currentWallet={currentWallet}
          newWalletName={newWalletName}
          setNewWalletName={setNewWalletName}
          mnemonic={mnemonic}
          setMnemonic={setMnemonic}
          loading={loading}
          createWallet={createWallet}
          importWallet={importWallet}
          loadWallet={loadWallet}
        />
      ) : (
        <div className="accounts-content">
          <div className="accounts-header">
            <div className="wallet-info">
              <h3>Wallet: {currentWallet}</h3>
              <button 
                className="change-wallet-btn"
                onClick={returnToWalletSelection}
                disabled={loading}
              >
                Change Wallet
              </button>
            </div>
            <div className="account-actions">
              <button 
                className="create-account-btn"
                onClick={createAccount}
                disabled={loading}
              >
                {loading ? "Creating..." : "Create New Account"}
              </button>
              <button 
                className="import-account-btn"
                onClick={() => setShowImportForm(true)}
                disabled={loading}
              >
                Import Account
              </button>
            </div>
          </div>
          
          <WalletOverview
            selectAccount={selectAccount}
            accounts={accounts}
          />
        </div>
      )}

      {showImportForm && (
        <div className="import-form-overlay">
          <div className="import-form">
            <div className="import-form-header">
              <h3>Import Account</h3>
              <button 
                className="close-btn"
                onClick={() => setShowImportForm(false)}
              >
                ✕
              </button>
            </div>
            
            <div className="form-section">
              <label htmlFor="accountName">Account Name</label>
              <input
                id="accountName"
                type="text"
                placeholder="Enter account name (e.g., My Account 1)"
                disabled={loading}
              />
            </div>

            <div className="form-section">
              <label htmlFor="mnemonic">Mnemonic Phrase</label>
              <textarea
                id="mnemonic"
                placeholder="Enter 12, 15, 18, 21, or 24 word mnemonic phrase"
                disabled={loading}
                rows={4}
              />
              <p className="input-help">
                Enter the mnemonic phrase for the account you want to import. 
                The phrase should contain 12, 15, 18, 21, or 24 words separated by spaces.
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
                  addAccount();
                  setShowImportForm(false);
                }}
                disabled={loading}
                className="import-btn"
              >
                {loading ? "Importing..." : "Import Account"}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AccountsPage;
