import React from 'react';
import { MenuItem } from '../types';
import AccountsPage from './AccountsPage';
import SendPage from './SendPage';
import ReceivePage from './ReceivePage';
import TransactionsPage from './TransactionsPage';
import AdvancedPage from './AdvancedPage';
import SettingsPage from './SettingsPage';

interface MainContentProps {
  activeMenu: MenuItem;
  message: string;
  setMessage: (message: string) => void;
  // Accounts page props
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
  walletInfo: any;
  // Send page props
  sendAmount: string;
  setSendAmount: (amount: string) => void;
  sendAddress: string;
  setSendAddress: (address: string) => void;
  transactionData: string;
  setTransactionData: (data: string) => void;
  signTransaction: () => void;
  signature: string;
  // Settings page props
  menuCollapsed: boolean;
  setMenuCollapsed: (collapsed: boolean) => void;
}

const MainContent: React.FC<MainContentProps> = ({
  activeMenu,
  message,
  setMessage,
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
  walletInfo,
  sendAmount,
  setSendAmount,
  sendAddress,
  setSendAddress,
  transactionData,
  setTransactionData,
  signTransaction,
  signature,
  menuCollapsed,
  setMenuCollapsed,
}) => {
  const renderContent = () => {
    switch (activeMenu) {
      case 'accounts':
        return (
          <AccountsPage
            currentWallet={currentWallet}
            wallets={wallets}
            newWalletName={newWalletName}
            setNewWalletName={setNewWalletName}
            mnemonic={mnemonic}
            setMnemonic={setMnemonic}
            loading={loading}
            createWallet={createWallet}
            importWallet={importWallet}
            loadWallet={loadWallet}
            returnToWalletSelection={returnToWalletSelection}
            createAccount={createAccount}
            addAccount={addAccount}
            selectAccount={selectAccount}
            accounts={accounts}
          />
        );

      case 'send':
        return (
          <SendPage
            sendAmount={sendAmount}
            setSendAmount={setSendAmount}
            sendAddress={sendAddress}
            setSendAddress={setSendAddress}
            transactionData={transactionData}
            setTransactionData={setTransactionData}
            loading={loading}
            signTransaction={signTransaction}
            signature={signature}
          />
        );
      case 'receive':
        return (
          <ReceivePage
            walletInfo={walletInfo}
          />
        );
      case 'transactions':
        return (
          <TransactionsPage />
        );
      case 'advanced':
        return (
          <AdvancedPage
            transactionData={transactionData}
            setTransactionData={setTransactionData}
            loading={loading}
            signTransaction={signTransaction}
            signature={signature}
          />
        );
      case 'settings':
        return (
          <SettingsPage
            currentWallet={currentWallet}
            walletInfo={walletInfo}
            menuCollapsed={menuCollapsed}
            setMenuCollapsed={setMenuCollapsed}
          />
        );
      default:
        return (
          <AccountsPage
            currentWallet={currentWallet}
            wallets={wallets}
            newWalletName={newWalletName}
            setNewWalletName={setNewWalletName}
            mnemonic={mnemonic}
            setMnemonic={setMnemonic}
            loading={loading}
            createWallet={createWallet}
            importWallet={importWallet}
            loadWallet={loadWallet}
            returnToWalletSelection={returnToWalletSelection}
            createAccount={createAccount}
            addAccount={addAccount}
            selectAccount={selectAccount}
            accounts={accounts}
          />
        );
    }
  };

  return (
    <div className="main-content">
      {message && (
        <div className="content-header">
          <div className="message">
            <p>{message}</p>
            <button 
              className="message-close-btn"
              onClick={() => setMessage ("")}
              title="Close message"
            >
              ✕
            </button>
          </div>
        </div>
      )}

      <div className="content-body">
        {renderContent()}
      </div>
    </div>
  );
};

export default MainContent;
