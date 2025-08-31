export interface AccountInfo {
  address: string;
  public_key: string;
  is_selected: boolean;
  created_at: string;
}

export interface WalletInfo {
  total_accounts: number;
  selected_account: string | null;
  created_at: string;
  last_modified: string;
}

export interface CreateWalletResponse {
  success: boolean;
  mnemonic: string;
  address: string;
  public_key: string;
  message: string;
}

export interface ImportWalletResponse {
  success: boolean;
  address: string;
  public_key: string;
  total_accounts: number;
  message: string;
}

export interface SignTransactionResponse {
  success: boolean;
  signature: string;
  message: string;
}

export type MenuItem = 'accounts' | 'send' | 'receive' | 'transactions' | 'advanced' | 'settings';

export type Theme = 'dark' | 'light';
