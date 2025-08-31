import React, { useEffect, useState } from 'react';
import QRCode from 'qrcode';

interface ReceivePageProps {
  walletInfo: any;
}

const ReceivePage: React.FC<ReceivePageProps> = ({
  walletInfo,
}) => {
  const [qrCodeDataUrl, setQrCodeDataUrl] = useState<string>('');

  useEffect(() => {
    if (walletInfo?.selected_account) {
      // 生成二维码
      QRCode.toDataURL(walletInfo.selected_account, {
        width: 200,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#FFFFFF'
        }
      })
      .then(url => {
        setQrCodeDataUrl(url);
      })
      .catch(err => {
        console.error('Error generating QR code:', err);
      });
    }
  }, [walletInfo?.selected_account]);

  return (
    <div className="receive-page">
      <div className="page-header">
        <h2>Receive</h2>
      </div>

      <div className="receive-content">
        {walletInfo?.selected_account ? (
          <div className="address-display">
            <h3>Your Address</h3>
            
            {/* QR Code Section */}
            <div className="qr-code-section">
              <h4>QR Code</h4>
              <div className="qr-code-container">
                {qrCodeDataUrl ? (
                  <img 
                    src={qrCodeDataUrl} 
                    alt="Address QR Code" 
                    className="qr-code-image"
                  />
                ) : (
                  <div className="qr-code-loading">Generating QR Code...</div>
                )}
              </div>
              <p className="qr-code-note">
                Scan this QR code to get the address
              </p>
            </div>

            {/* Address Section */}
            <div className="address-section">
              <h4>Address</h4>
              <div className="address-box">
                <code>{walletInfo.selected_account}</code>
                <button
                  onClick={() => navigator.clipboard.writeText(walletInfo.selected_account!)}
                  className="copy-button"
                >
                  Copy Address
                </button>
              </div>
              <p className="address-note">
                Share this address with others to receive Mazzaroth tokens.
              </p>
            </div>
          </div>
        ) : (
          <div className="no-account">
            <h3>No Account Selected</h3>
            <p>Please select an account from the Accounts page to view your address.</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ReceivePage;
