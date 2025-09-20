import React from 'react';
import { MenuItem, AccountInfo } from '../types';

interface SidebarProps {
  menuCollapsed: boolean;
  setMenuCollapsed: (collapsed: boolean) => void;
  activeMenu: MenuItem;
  setActiveMenu: (menu: MenuItem) => void;
  currentWallet?: string;
  selectedAccount?: AccountInfo | null;
}

const Sidebar: React.FC<SidebarProps> = ({
  menuCollapsed,
  setMenuCollapsed,
  activeMenu,
  setActiveMenu,
  currentWallet,
  selectedAccount,
}) => {
  return (
    <div className={`sidebar ${menuCollapsed ? 'collapsed' : ''}`}>
      <div className="sidebar-header">
        {currentWallet ? (
          <div className="wallet-info-header">
            {menuCollapsed ? (
              <div className="wallet-info-collapsed">
                <div className="wallet-account-card-collapsed">
                  <div className="wallet-icon">💎</div>
                  <div className="account-icon">👤</div>
                </div>
              </div>
            ) : (
              <div className="wallet-account-card">
                <div className="card-header">
                  <span className="wallet-icon">💎</span>
                  <span className="wallet-title">{currentWallet}</span>
                </div>
                {selectedAccount && (
                  <div className="card-content">
                    <span className="account-icon">👤</span>
                    <span className="account-title">
                      {selectedAccount.address.slice(0, 8)}...{selectedAccount.address.slice(-6)}
                    </span>
                  </div>
                )}
              </div>
            )}
          </div>
        ) : (
          <h1 className={menuCollapsed ? "collapsed" : ""}>
            {menuCollapsed ? "MW" : "Mazzaroth Wallet"}
          </h1>
        )}
      </div>

      <nav className="sidebar-nav">
        <button
          className={`nav-item ${activeMenu === 'accounts' ? 'active' : ''}`}
          onClick={() => setActiveMenu('accounts')}
        >
          <span className="nav-icon">👤</span>
          {!menuCollapsed && <span>Accounts</span>}
        </button>
        <button
          className={`nav-item ${activeMenu === 'send' ? 'active' : ''}`}
          onClick={() => setActiveMenu('send')}
        >
          <span className="nav-icon">📤</span>
          {!menuCollapsed && <span>Send</span>}
        </button>
        <button
          className={`nav-item ${activeMenu === 'receive' ? 'active' : ''}`}
          onClick={() => setActiveMenu('receive')}
        >
          <span className="nav-icon">📥</span>
          {!menuCollapsed && <span>Receive</span>}
        </button>
        <button
          className={`nav-item ${activeMenu === 'transactions' ? 'active' : ''}`}
          onClick={() => setActiveMenu('transactions')}
        >
          <span className="nav-icon">📋</span>
          {!menuCollapsed && <span>Transactions</span>}
        </button>
        <button
          className={`nav-item ${activeMenu === 'advanced' ? 'active' : ''}`}
          onClick={() => setActiveMenu('advanced')}
        >
          <span className="nav-icon">⚙️</span>
          {!menuCollapsed && <span>Advanced</span>}
        </button>
        <button
          className={`nav-item ${activeMenu === 'settings' ? 'active' : ''}`}
          onClick={() => setActiveMenu('settings')}
        >
          <span className="nav-icon">🔧</span>
          {!menuCollapsed && <span>Settings</span>}
        </button>
      </nav>

      <div className="sidebar-footer">
        <button
          className="menu-toggle"
          onClick={() => setMenuCollapsed(!menuCollapsed)}
          title={menuCollapsed ? "Expand menu" : "Collapse menu"}
        >
          <span className={`arrow-icon ${menuCollapsed ? 'collapsed' : ''}`}>
            {menuCollapsed ? '→' : '←'}
          </span>
        </button>
      </div>
    </div>
  );
};

export default Sidebar;
