import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Sidebar from "./components/Sidebar";
import MainContent from "./components/MainContent";
import TopMenuBar from "./components/TopMenuBar";
import ErrorBoundary from "./components/ErrorBoundary";
import { MenuItem, AccountInfo, WalletInfo, CreateWalletResponse, ImportWalletResponse, SignTransactionResponse } from "./types";
import { ThemeProvider } from "./contexts/ThemeContext";

function App() {
  const [wallets, setWallets] = useState<string[]>([]);
  const [currentWallet, setCurrentWallet] = useState<string>("");
  const [accounts, setAccounts] = useState<AccountInfo[]>([]);
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);
  const [mnemonic, setMnemonic] = useState<string>("");
  const [newWalletName, setNewWalletName] = useState<string>("");
  const [transactionData, setTransactionData] = useState<string>("");
  const [signature, setSignature] = useState<string>("");
  const [message, setMessage] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [menuCollapsed, setMenuCollapsed] = useState<boolean>(false);
  const [activeMenu, setActiveMenu] = useState<MenuItem>('accounts');

  // 处理菜单切换，同时清除消息
  const handleMenuChange = (menu: MenuItem) => {
    setActiveMenu(menu);
    setMessage(""); // 清除错误消息
  };
  const [sendAmount, setSendAmount] = useState<string>("");
  const [sendAddress, setSendAddress] = useState<string>("");

  useEffect(() => {
    loadWallets();
  }, []);

  async function loadWallets() {
    try {
      setLoading(true);
      const walletList = await invoke<string[]>("list_wallets");
      setWallets(walletList);
      setMessage("Wallets loaded successfully");
    } catch (error) {
      setMessage(`Error loading wallets: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function createWallet() {
    if (!newWalletName.trim()) {
      setMessage("Please enter a wallet name");
      return;
    }

    try {
      setLoading(true);
      const response: CreateWalletResponse = await invoke("create_wallet", {
        walletName: newWalletName,
      });

      if (response.success) {
        setMessage(`Wallet created successfully! Mnemonic: ${response.mnemonic}`);
        setNewWalletName("");
        await loadWallets();
      } else {
        setMessage("Failed to create wallet");
      }
    } catch (error) {
      setMessage(`Error creating wallet: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function importWallet() {
    if (!newWalletName.trim() || !mnemonic.trim()) {
      setMessage("Please enter both wallet name and mnemonic");
      return;
    }

    try {
      setLoading(true);
      const response: ImportWalletResponse = await invoke("import_wallet", {
        walletName: newWalletName,
        mnemonic: mnemonic,
      });

      if (response.success) {
        setMessage("Wallet imported successfully!");
        setNewWalletName("");
        setMnemonic("");
        await loadWallets();
      } else {
        setMessage("Failed to import wallet");
      }
    } catch (error) {
      setMessage(`Error importing wallet: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function loadWallet(walletName: string) {
    try {
      setLoading(true);
      console.log('Loading wallet:', walletName);
      const success = await invoke<boolean>("load_wallet", {
        walletName: walletName,
      });

      console.log('Load wallet result:', success);
      if (success) {
        setCurrentWallet(walletName);
        console.log('Wallet loaded, loading accounts...');
        await loadAccounts();
        console.log('Accounts loaded, loading wallet info...');
        await loadWalletInfo();
        console.log('Wallet info loaded');
      } else {
        setMessage("Failed to load wallet");
      }
    } catch (error) {
      console.error('Error loading wallet:', error);
      setMessage(`Error loading wallet: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  function returnToWalletSelection() {
    setCurrentWallet("");
    setAccounts([]);
    setWalletInfo(null);
    setMnemonic("");
  }



  async function loadAccounts() {
    try {
      console.log('Loading accounts...');
      const accountList = await invoke<AccountInfo[]>("list_accounts");
      console.log('Accounts loaded:', accountList);
      setAccounts(accountList);
    } catch (error) {
      console.error('Error loading accounts:', error);
      setMessage(`Error loading accounts: ${error}`);
    }
  }

  async function loadWalletInfo() {
    try {
      console.log('Loading wallet info...');
      const info = await invoke<WalletInfo | null>("get_wallet_info");
      console.log('Wallet info loaded:', info);
      setWalletInfo(info);
    } catch (error) {
      console.error('Error loading wallet info:', error);
      setMessage(`Error loading wallet info: ${error}`);
    }
  }

  async function selectAccount(index: number) {
    try {
      await invoke("select_account", { index });
      await loadAccounts();
      await loadWalletInfo();
      setMessage("Account selected successfully");
    } catch (error) {
      setMessage(`Error selecting account: ${error}`);
    }
  }

    async function addAccount() {
    if (!mnemonic.trim()) {
      setMessage("Please enter a mnemonic phrase");
      return;
    }

    // Validate mnemonic word count
    const wordCount = mnemonic.trim().split(/\s+/).length;
    const validWordCounts = [12, 15, 18, 21, 24];

    if (!validWordCounts.includes(wordCount)) {
      setMessage(`Invalid mnemonic word count: ${wordCount}. Must be 12, 15, 18, 21, or 24 words.`);
      return;
    }

    try {
      setLoading(true);
      await invoke("add_account", { mnemonic: mnemonic.trim() });
      await loadAccounts();
      await loadWalletInfo();
      setMnemonic("");
      setMessage("Account added successfully");
    } catch (error) {
      setMessage(`Error adding account: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function createAccount() {
    try {
      setLoading(true);
      await invoke("create_account");
      await loadAccounts();
      await loadWalletInfo();
      setMessage("Account created successfully");
    } catch (error) {
      setMessage(`Error creating account: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  async function signTransaction() {
    if (!transactionData.trim()) {
      setMessage("Please enter transaction data");
      return;
    }

    try {
      setLoading(true);
      const response = await invoke<SignTransactionResponse>("sign_transaction", {
        data: transactionData,
      });
      setSignature(response.signature);
      setMessage("Transaction signed successfully");
    } catch (error) {
      setMessage(`Error signing transaction: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  return (
    <ThemeProvider>
      <ErrorBoundary>
        <div className="app">
          <TopMenuBar />
          <div className="app-content">
            <Sidebar
              menuCollapsed={menuCollapsed}
              setMenuCollapsed={setMenuCollapsed}
              activeMenu={activeMenu}
              setActiveMenu={handleMenuChange}
            />
            <MainContent
              activeMenu={activeMenu}
              message={message}
              setMessage={setMessage}
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
              addAccount={addAccount}
              createAccount={createAccount}
              selectAccount={selectAccount}
              accounts={accounts}
              walletInfo={walletInfo}
              sendAmount={sendAmount}
              setSendAmount={setSendAmount}
              sendAddress={sendAddress}
              setSendAddress={setSendAddress}
              transactionData={transactionData}
              setTransactionData={setTransactionData}
              signTransaction={signTransaction}
              signature={signature}
              menuCollapsed={menuCollapsed}
              setMenuCollapsed={setMenuCollapsed}
            />
          </div>
        </div>
      </ErrorBoundary>
    </ThemeProvider>
  );
}

export default App;
