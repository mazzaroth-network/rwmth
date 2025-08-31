import React from 'react';
import { useTheme } from '../contexts/ThemeContext';

interface SettingsPageProps {
  currentWallet: string;
  walletInfo: any;
  menuCollapsed: boolean;
  setMenuCollapsed: (collapsed: boolean) => void;
}

const SettingsPage: React.FC<SettingsPageProps> = ({
  currentWallet,
  walletInfo,
  menuCollapsed,
  setMenuCollapsed,
}) => {
  const { theme, toggleTheme } = useTheme();

  return (
    <div className="settings-page">
      <div className="page-header">
        <h2>Settings</h2>
      </div>

      <div className="settings-content">
        <div className="settings-section">
          <h3>Wallet Settings</h3>
          <div className="setting-item">
            <label>Current Wallet</label>
            <span>{currentWallet || "None"}</span>
          </div>
          <div className="setting-item">
            <label>Total Accounts</label>
            <span>{walletInfo?.total_accounts || 0}</span>
          </div>
        </div>

        <div className="settings-section">
          <h3>Application Settings</h3>
          <div className="setting-item">
            <label>Theme</label>
            <div className="theme-toggle">
              <span className="theme-label">{theme === 'dark' ? '🌙' : '☀️'}</span>
              <button onClick={toggleTheme} className="theme-btn">
                {theme === 'dark' ? 'Dark Mode' : 'Light Mode'}
              </button>
            </div>
          </div>
          <div className="setting-item">
            <label>Menu Collapsed</label>
            <button onClick={() => setMenuCollapsed(!menuCollapsed)}>
              {menuCollapsed ? "Expand" : "Collapse"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SettingsPage;
