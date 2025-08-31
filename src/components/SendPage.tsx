import React from 'react';

interface SendPageProps {
  sendAmount: string;
  setSendAmount: (amount: string) => void;
  sendAddress: string;
  setSendAddress: (address: string) => void;
  transactionData: string;
  setTransactionData: (data: string) => void;
  loading: boolean;
  signTransaction: () => void;
  signature: string;
}

const SendPage: React.FC<SendPageProps> = ({
  sendAmount,
  setSendAmount,
  sendAddress,
  setSendAddress,
  transactionData,
  setTransactionData,
  loading,
  signTransaction,
  signature,
}) => {
  return (
    <div className="send-page">
      <div className="page-header">
        <h2>Send</h2>
      </div>

      <div className="send-form">
        <div className="form-group">
          <label>Amount</label>
          <input
            type="text"
            placeholder="Enter amount"
            value={sendAmount}
            onChange={(e) => setSendAmount(e.target.value)}
          />
        </div>

        <div className="form-group">
          <label>Recipient Address</label>
          <input
            type="text"
            placeholder="Enter recipient address"
            value={sendAddress}
            onChange={(e) => setSendAddress(e.target.value)}
          />
        </div>

        <div className="form-group">
          <label>Transaction Data (Hex)</label>
          <input
            type="text"
            placeholder="Transaction data (hex format)"
            value={transactionData}
            onChange={(e) => setTransactionData(e.target.value)}
          />
        </div>

        <button onClick={signTransaction} disabled={loading} className="send-btn">
          {loading ? "Signing..." : "Sign & Send Transaction"}
        </button>

        {signature && (
          <div className="signature-result">
            <h3>Transaction Signed</h3>
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
  );
};

export default SendPage;
