import React from 'react';
import { useTheme } from '../contexts/ThemeContext';

// 声明 Tauri 全局类型
declare global {
  interface Window {
    __TAURI__?: {
      window: {
        getCurrent: () => {
          minimize: () => void;
          toggleMaximize: () => void;
          close: () => void;
        };
      };
    };
  }
}

const TopMenuBar: React.FC = () => {
  const { theme, toggleTheme } = useTheme();

  const handleMinimize = () => {
    // 使用 Tauri API 最小化窗口
    if (window.__TAURI__) {
      window.__TAURI__.window.getCurrent().minimize();
    }
  };

  const handleMaximize = () => {
    // 使用 Tauri API 最大化/还原窗口
    if (window.__TAURI__) {
      window.__TAURI__.window.getCurrent().toggleMaximize();
    }
  };

  const handleClose = () => {
    // 使用 Tauri API 关闭窗口
    if (window.__TAURI__) {
      window.__TAURI__.window.getCurrent().close();
    }
  };

  return (
    <div className="top-menu-bar">
      <div className="top-menu-left">
        <div className="app-title">
          <span className="app-icon">💎</span>
          <span className="app-name">Mazzaroth Wallet</span>
        </div>
      </div>
      
      <div className="top-menu-center">
        <div className="menu-actions">
          <button 
            className="menu-action-btn theme-toggle-btn"
            onClick={toggleTheme}
            title={`Switch to ${theme === 'dark' ? 'Light' : 'Dark'} Mode`}
          >
            {theme === 'dark' ? '☀️' : '🌙'}
          </button>
        </div>
      </div>

      <div className="top-menu-right">
        <div className="window-controls">
          <button 
            className="window-control-btn minimize-btn"
            onClick={handleMinimize}
            title="Minimize"
          >
            <span>─</span>
          </button>
          <button 
            className="window-control-btn maximize-btn"
            onClick={handleMaximize}
            title="Maximize"
          >
            <span>□</span>
          </button>
          <button 
            className="window-control-btn close-btn"
            onClick={handleClose}
            title="Close"
          >
            <span>×</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default TopMenuBar;
