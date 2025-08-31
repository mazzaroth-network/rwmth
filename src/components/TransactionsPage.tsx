import React from 'react';

interface TransactionsPageProps {
}

const TransactionsPage: React.FC<TransactionsPageProps> = ({}) => {
  return (
    <div className="transactions-page">
      <div className="page-header">
        <h2>Transactions</h2>
      </div>

      <div className="transactions-content">
        <div className="no-transactions">
          <h3>No Transactions Yet</h3>
          <p>Transaction history will appear here once you start using your wallet.</p>
        </div>
      </div>
    </div>
  );
};

export default TransactionsPage;
