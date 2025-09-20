import React, { useState } from 'react';
import { useTheme } from '../contexts/ThemeContext';

interface SettingsPageProps {
  currentWallet: string;
  walletInfo: any;
  menuCollapsed: boolean;
  setMenuCollapsed: (collapsed: boolean) => void;
}

type SettingsTab = 'wallet' | 'interface' | 'node' | 'log' | 'info';

const SettingsPage: React.FC<SettingsPageProps> = ({
  currentWallet,
  walletInfo,
  menuCollapsed,
  setMenuCollapsed,
}) => {
  const { theme, toggleTheme } = useTheme();
  const [activeTab, setActiveTab] = useState<SettingsTab>('wallet');

  const tabs = [
    { id: 'wallet' as SettingsTab, label: 'Wallet', icon: '💼' },
    { id: 'interface' as SettingsTab, label: 'Interface', icon: '🎨' },
    { id: 'node' as SettingsTab, label: 'Node', icon: '🌐' },
    { id: 'log' as SettingsTab, label: 'Log', icon: '📋' },
    { id: 'info' as SettingsTab, label: 'Info', icon: 'ℹ️' },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case 'wallet':
        return (
          <div className="tab-content">
            <div className="settings-section">
              <h3>Wallet Configuration</h3>
              <div className="setting-item">
                <label>Current Wallet</label>
                <span>{currentWallet || "None"}</span>
              </div>
              <div className="setting-item">
                <label>Total Accounts</label>
                <span>{walletInfo?.total_accounts || 0}</span>
              </div>
              <div className="setting-item">
                <label>Selected Account</label>
                <span>{walletInfo?.selected_account || "None"}</span>
              </div>
            </div>
            <div className="settings-section">
              <h3>Wallet Management</h3>
              <div className="setting-item">
                <label>Auto-lock Timeout</label>
                <select className="setting-select">
                  <option value="5">5 minutes</option>
                  <option value="15">15 minutes</option>
                  <option value="30">30 minutes</option>
                  <option value="60">1 hour</option>
                  <option value="0">Never</option>
                </select>
              </div>
              <div className="setting-item">
                <label>Backup Frequency</label>
                <select className="setting-select">
                  <option value="daily">Daily</option>
                  <option value="weekly">Weekly</option>
                  <option value="monthly">Monthly</option>
                  <option value="manual">Manual Only</option>
                </select>
              </div>
            </div>
          </div>
        );

      case 'interface':
        return (
          <div className="tab-content">
            <div className="settings-section">
              <h3>Appearance</h3>
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
                <label>Language</label>
                <select className="setting-select">
                  <option value="en">English</option>
                  <option value="zh">中文</option>
                  <option value="ja">日本語</option>
                </select>
              </div>
            </div>
            <div className="settings-section">
              <h3>Layout</h3>
              <div className="setting-item">
                <label>Menu Collapsed</label>
                <button onClick={() => setMenuCollapsed(!menuCollapsed)}>
                  {menuCollapsed ? "Expand" : "Collapse"}
                </button>
              </div>
              <div className="setting-item">
                <label>Default View</label>
                <select className="setting-select">
                  <option value="accounts">Accounts</option>
                  <option value="send">Send</option>
                  <option value="receive">Receive</option>
                </select>
              </div>
            </div>
          </div>
        );

      case 'node':
        return (
          <div className="tab-content">
            <div className="settings-section">
              <h3>Network Configuration</h3>
              <div className="setting-item">
                <label>Node URL</label>
                <input 
                  type="text" 
                  className="setting-input" 
                  placeholder="https://node.mazzaroth.io"
                  defaultValue="https://node.mazzaroth.io"
                />
              </div>
              <div className="setting-item">
                <label>Network</label>
                <select className="setting-select">
                  <option value="mainnet">Mainnet</option>
                  <option value="testnet">Testnet</option>
                  <option value="devnet">Devnet</option>
                </select>
              </div>
            </div>
            <div className="settings-section">
              <h3>Connection Settings</h3>
              <div className="setting-item">
                <label>Connection Timeout</label>
                <input 
                  type="number" 
                  className="setting-input" 
                  placeholder="30"
                  defaultValue="30"
                />
                <span className="setting-unit">seconds</span>
              </div>
              <div className="setting-item">
                <label>Auto-reconnect</label>
                <input type="checkbox" className="setting-checkbox" defaultChecked />
              </div>
            </div>
          </div>
        );

      case 'log':
        return (
          <div className="tab-content">
            <div className="settings-section">
              <h3>Logging Configuration</h3>
              <div className="setting-item">
                <label>Log Level</label>
                <select className="setting-select">
                  <option value="error">Error</option>
                  <option value="warn">Warning</option>
                  <option value="info">Info</option>
                  <option value="debug">Debug</option>
                  <option value="trace">Trace</option>
                </select>
              </div>
              <div className="setting-item">
                <label>Log to File</label>
                <input type="checkbox" className="setting-checkbox" defaultChecked />
              </div>
              <div className="setting-item">
                <label>Max Log File Size</label>
                <input 
                  type="number" 
                  className="setting-input" 
                  placeholder="10"
                  defaultValue="10"
                />
                <span className="setting-unit">MB</span>
              </div>
            </div>
            <div className="settings-section">
              <h3>Log Management</h3>
              <div className="setting-item">
                <label>Clear Logs</label>
                <button className="danger-btn">Clear All Logs</button>
              </div>
              <div className="setting-item">
                <label>Export Logs</label>
                <button className="secondary-btn">Export to File</button>
              </div>
            </div>
          </div>
        );

      case 'info':
        return (
          <div className="tab-content">
            <div className="settings-section">
              <h3>Application Information</h3>
              <div className="setting-item">
                <label>GUI Version</label>
                <span className="info-value">
                  {import.meta.env.VITE_APP_VERSION || '0.1.0'}
                </span>
              </div>
              <div className="setting-item">
                <label>Embedded Mazzaroth Version</label>
                <span className="info-value">
                  {import.meta.env.VITE_MAZZAROTH_VERSION || '0.8.2'}
                </span>
              </div>
              <div className="setting-item">
                <label>Build Date</label>
                <span className="info-value">
                  {import.meta.env.VITE_BUILD_DATE || 
                   new Date(document.lastModified).toLocaleDateString('en-CA')}
                </span>
              </div>
            </div>
            <div className="settings-section">
              <h3>Paths</h3>
              <div className="setting-item">
                <label>Wallet Path</label>
                <div className="path-display">
                  <span className="path-value">~/Library/Application Support/MazzarothWallet/wallets/</span>
                  <button className="copy-btn" title="Copy path">📋</button>
                </div>
              </div>
              <div className="setting-item">
                <label>Wallet Log Path</label>
                <div className="path-display">
                  <span className="path-value">~/Library/Logs/MazzarothWallet/</span>
                  <button className="copy-btn" title="Copy path">📋</button>
                </div>
              </div>
              <div className="setting-item">
                <label>Config Path</label>
                <div className="path-display">
                  <span className="path-value">~/Library/Application Support/MazzarothWallet/config/</span>
                  <button className="copy-btn" title="Copy path">📋</button>
                </div>
              </div>
            </div>
            <div className="settings-section">
              <h3>System Information</h3>
              <div className="setting-item">
                <label>Platform</label>
                <span className="info-value">macOS 14.6.0</span>
              </div>
              <div className="setting-item">
                <label>Architecture</label>
                <span className="info-value">x86_64</span>
              </div>
              <div className="setting-item">
                <label>Memory Usage</label>
                <span className="info-value">45.2 MB</span>
              </div>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="settings-page">
      <div className="page-header">
        <h2>Settings</h2>
      </div>

      <div className="settings-content">
        <div className="settings-tabs">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`settings-tab ${activeTab === tab.id ? 'active' : ''}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <span className="tab-icon">{tab.icon}</span>
              <span className="tab-label">{tab.label}</span>
            </button>
          ))}
        </div>

        <div className="settings-panel">
          {renderTabContent()}
        </div>
      </div>
    </div>
  );
};

export default SettingsPage;
