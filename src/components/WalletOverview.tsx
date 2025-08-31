import React, { useState } from 'react';
import { AccountInfo } from '../types';

// Mazzaroth coin type for BIP44 derivation
const COIN_TYPE = 55555;

interface WalletOverviewProps {
  selectAccount: (index: number) => void;
  accounts: AccountInfo[];
}

const WalletOverview: React.FC<WalletOverviewProps> = ({
  selectAccount,
  accounts,
}) => {
  const [selectedAccountIndex, setSelectedAccountIndex] = useState<number | null>(null);

  const handleAccountClick = (index: number) => {
    setSelectedAccountIndex(selectedAccountIndex === index ? null : index);
  };

  const selectedAccount = selectedAccountIndex !== null ? accounts[selectedAccountIndex] : null;

  return (
    <div className="wallet-overview">
      <div className="accounts-section">
        <div className="accounts-header">
          <h3>Your Accounts ({accounts.length})</h3>
          <div className="account-actions">
            <button 
              className="action-btn select-btn"
              onClick={() => selectedAccountIndex !== null && selectAccount(selectedAccountIndex)}
              disabled={selectedAccountIndex === null}
            >
              Set as Active
            </button>
          </div>
        </div>

        {accounts.length === 0 ? (
          <div className="no-accounts">
            <p>No accounts found. Add an account above.</p>
          </div>
        ) : (
          <div className="accounts-container">
            {/* Account List - Top Section */}
            <div className="account-list-section">
              <h4>Account List</h4>
              <div className="account-list">
                {accounts.map((account, index) => (
                  <div
                    key={account.address}
                    className={`account-list-item ${account.is_selected ? "active" : ""} ${selectedAccountIndex === index ? "selected" : ""}`}
                    onClick={() => handleAccountClick(index)}
                  >
                    <div className="account-list-header">
                      <span className="account-name">Account {index + 1}</span>
                      <span className="account-path">m/44'/{COIN_TYPE}'/0'/0/{index}</span>
                      {account.is_selected && <span className="active-badge">Active</span>}
                      {selectedAccountIndex === index && <span className="selected-badge">Selected</span>}
                    </div>
                    <div className="account-list-address">
                      {account.address.substring(0, 8)}...{account.address.substring(account.address.length - 8)}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Account Details - Bottom Section */}
            {selectedAccount && (
              <div className="account-details-section">
                <h4>Account Details</h4>
                <div className="account-details-card">
                  <div className="detail-group">
                    <label>Account Index</label>
                    <span>{selectedAccountIndex! + 1}</span>
                  </div>
                  <div className="detail-group">
                    <label>BIP44 Path</label>
                    <span className="bip44-path">m/44'/{COIN_TYPE}'/0'/0/{selectedAccountIndex}</span>
                  </div>
                  <div className="detail-group">
                    <label>Address</label>
                    <div className="detail-value">
                      <code>{selectedAccount.address}</code>
                      <button
                        onClick={() => navigator.clipboard.writeText(selectedAccount.address)}
                        className="copy-button"
                        title="Copy address"
                      >
                        📋
                      </button>
                    </div>
                  </div>
                  <div className="detail-group">
                    <label>Public Key</label>
                    <div className="detail-value">
                      <code>{selectedAccount.public_key}</code>
                      <button
                        onClick={() => navigator.clipboard.writeText(selectedAccount.public_key)}
                        className="copy-button"
                        title="Copy public key"
                      >
                        📋
                      </button>
                    </div>
                  </div>
                  <div className="detail-group">
                    <label>Status</label>
                    <span className={`status-badge ${selectedAccount.is_selected ? "active" : "inactive"}`}>
                      {selectedAccount.is_selected ? "Active" : "Inactive"}
                    </span>
                  </div>
                  <div className="detail-group">
                    <label>Created</label>
                    <span>{new Date(selectedAccount.created_at).toLocaleString()}</span>
                  </div>
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default WalletOverview;
