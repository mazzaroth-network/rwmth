import React from 'react';

interface AdvancedPageProps {
  transactionData: string;
  setTransactionData: (data: string) => void;
  loading: boolean;
  signTransaction: () => void;
  signature: string;
}

const AdvancedPage: React.FC<AdvancedPageProps> = ({
  transactionData,
  setTransactionData,
  loading,
  signTransaction,
  signature,
}) => {
  return (
    <div className="advanced-page">
      <div className="page-header">
        <h2>Advanced</h2>
      </div>

      <div className="advanced-content">
        <div className="advanced-section">
          <h3>Transaction Signing</h3>
          <div className="form-group">
            <label>Transaction Data (Hex)</label>
            <input
              type="text"
              placeholder="Enter transaction data in hex format"
              value={transactionData}
              onChange={(e) => setTransactionData(e.target.value)}
            />
          </div>
          <button onClick={signTransaction} disabled={loading}>
            {loading ? "Signing..." : "Sign Transaction"}
          </button>

          {signature && (
            <div className="signature-result">
              <h4>Signature</h4>
              <div className="signature-display">
                <code>{signature}</code>
                <button
                  onClick={() => navigator.clipboard.writeText(signature)}
                  className="copy-button"
                >
                  Copy
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default AdvancedPage;
