import React from 'react';
import { MenuItem } from '../types';

interface SidebarProps {
  menuCollapsed: boolean;
  setMenuCollapsed: (collapsed: boolean) => void;
  activeMenu: MenuItem;
  setActiveMenu: (menu: MenuItem) => void;
}

const Sidebar: React.FC<SidebarProps> = ({
  menuCollapsed,
  setMenuCollapsed,
  activeMenu,
  setActiveMenu,
}) => {
  return (
    <div className={`sidebar ${menuCollapsed ? 'collapsed' : ''}`}>
      <div className="sidebar-header">
        <h1 className={menuCollapsed ? "collapsed" : ""}>
          {menuCollapsed ? "MW" : "Mazzaroth Wallet"}
        </h1>
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
