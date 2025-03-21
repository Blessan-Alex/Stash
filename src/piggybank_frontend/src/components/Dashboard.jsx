import React, { useState, useEffect } from 'react';
import { useAuth } from '../context/AuthContext';
import { backendService } from '../services/backendService';
import './Dashboard.css';

const Dashboard = () => {
  const { principal, logout } = useAuth();
  const [balance, setBalance] = useState(null);
  const [depositAmount, setDepositAmount] = useState('');
  const [withdrawAmount, setWithdrawAmount] = useState('');
  const [selectedLockPeriod, setSelectedLockPeriod] = useState('ThreeMonths');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [showWarningModal, setShowWarningModal] = useState(false);
  const [showInsufficientBalanceModal, setShowInsufficientBalanceModal] = useState(false);
  const [selectedDeposit, setSelectedDeposit] = useState(null);
  const [transactionHistory, setTransactionHistory] = useState([]);

  useEffect(() => {
    fetchBalance();
    fetchTransactionHistory();
  }, []);

  const fetchTransactionHistory = async () => {
    try {
      const userBalance = await backendService.getBalance();
      if (userBalance?.deposits) {
        const deposits = userBalance.deposits.map(deposit => ({
          type: 'Deposit',
          amount: Number(deposit.amount),
          lockPeriod: Object.keys(deposit.lock_period)[0],
          interestRate: `${deposit.interest_rate}%`,
          date: new Date(Number(deposit.deposit_time) / 1000000),
          isEarlyWithdrawal: false
        }));
        setTransactionHistory(deposits);
      }
    } catch (err) {
      console.error('Error fetching transaction history:', err);
    }
  };

  const fetchBalance = async () => {
    try {
      setLoading(true);
      setError('');
      const userBalance = await backendService.getBalance();
      setBalance(userBalance);
    } catch (err) {
      console.error('Error fetching balance:', err);
      setError('Failed to fetch balance. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleDeposit = async (e) => {
    e.preventDefault();
    if (!depositAmount || isNaN(depositAmount) || Number(depositAmount) <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    try {
      setLoading(true);
      setError('');
      
      const amount = BigInt(Math.floor(Number(depositAmount)));
      console.log('Depositing amount:', amount.toString());
      
      await backendService.mintTokens(
        amount,
        { [selectedLockPeriod]: null }
      );

      // Add to transaction history
      const transaction = {
        type: 'Deposit',
        amount: Number(depositAmount),
        lockPeriod: selectedLockPeriod,
        interestRate: getInterestRate(selectedLockPeriod),
        date: new Date(),
        isEarlyWithdrawal: false
      };
      setTransactionHistory(prev => [transaction, ...prev]);

      // Update balance
      const currentBalance = balance || {
        total_balance: BigInt(0),
        locked_balance: BigInt(0),
        available_balance: BigInt(0),
        deposits: [],
        rewards_earned: BigInt(0)
      };

      setBalance({
        ...currentBalance,
        total_balance: currentBalance.total_balance + amount,
        locked_balance: currentBalance.locked_balance + amount,
        available_balance: currentBalance.available_balance + amount,
        deposits: [
          ...currentBalance.deposits,
          {
            amount: amount,
            lock_period: { [selectedLockPeriod]: null },
            deposit_time: BigInt(Date.now() * 1000000),
            interest_rate: Number(getInterestRate(selectedLockPeriod).replace('%', '')),
            early_withdrawal_penalty: BigInt(0)
          }
        ]
      });

      setDepositAmount('');
    } catch (err) {
      console.error('Error depositing:', err);
      setError('Failed to deposit. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleWithdraw = async (e) => {
    e.preventDefault();
    if (!withdrawAmount || isNaN(withdrawAmount) || Number(withdrawAmount) <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    try {
      setLoading(true);
      setError('');
      
      const withdrawAmountInBigInt = BigInt(Math.floor(Number(withdrawAmount)));

      // First check if user has sufficient total balance
      if (!balance?.total_balance || balance.total_balance < withdrawAmountInBigInt) {
        setShowInsufficientBalanceModal(true);
        setLoading(false);
        return;
      }

      // Find matching deposit
      const matchingDeposit = balance?.deposits?.find(deposit => 
        deposit.amount === withdrawAmountInBigInt
      );

      if (matchingDeposit) {
        const depositTime = Number(matchingDeposit.deposit_time) / 1000000;
        const currentTime = Date.now() / 1000;
        const lockPeriodInSeconds = {
          'ThreeMonths': 90 * 24 * 60 * 60,
          'SixMonths': 180 * 24 * 60 * 60,
          'TwelveMonths': 365 * 24 * 60 * 60
        }[Object.keys(matchingDeposit.lock_period)[0]];

        if (currentTime - depositTime < lockPeriodInSeconds) {
          setSelectedDeposit(matchingDeposit);
          setShowWarningModal(true);
          setLoading(false);
          return;
        }
      }

      // If we get here, it's a regular withdrawal
      await backendService.burnTokens(withdrawAmountInBigInt);
      
      // Update transaction history
      const transaction = {
        type: 'Withdrawal',
        amount: Number(withdrawAmount),
        lockPeriod: matchingDeposit ? Object.keys(matchingDeposit.lock_period)[0] : '-',
        interestRate: matchingDeposit ? getInterestRate(Object.keys(matchingDeposit.lock_period)[0]) : '-',
        date: new Date(),
        isEarlyWithdrawal: false
      };
      setTransactionHistory(prev => [transaction, ...prev]);

      // Update local balance state
      const currentBalance = balance;
      setBalance({
        ...currentBalance,
        total_balance: currentBalance.total_balance - withdrawAmountInBigInt,
        locked_balance: currentBalance.locked_balance - withdrawAmountInBigInt,
        available_balance: currentBalance.available_balance - withdrawAmountInBigInt,
        deposits: currentBalance.deposits.filter(deposit => 
          deposit.amount !== withdrawAmountInBigInt
        )
      });

      setWithdrawAmount('');
    } catch (err) {
      console.error('Error withdrawing:', err);
      setError('Failed to withdraw. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const confirmEarlyWithdraw = async () => {
    try {
      setLoading(true);
      setError('');
      
      const withdrawAmountInBigInt = BigInt(selectedDeposit.amount);
      const penaltyAmount = BigInt(Math.floor(Number(selectedDeposit.amount) * 0.1));
      const totalWithdrawAmount = withdrawAmountInBigInt + penaltyAmount;
      
      // Check if user still has sufficient balance including penalty
      if (!balance?.total_balance || balance.total_balance < totalWithdrawAmount) {
        setError('Insufficient balance for withdrawal including penalty');
        setShowWarningModal(false);
        setLoading(false);
        return;
      }

      await backendService.burnTokens(totalWithdrawAmount);
      
      // Update transaction history
      const transaction = {
        type: 'Withdrawal',
        amount: Number(selectedDeposit.amount),
        lockPeriod: Object.keys(selectedDeposit.lock_period)[0],
        interestRate: getInterestRate(Object.keys(selectedDeposit.lock_period)[0]),
        date: new Date(),
        isEarlyWithdrawal: true,
        penalty: Number(penaltyAmount)
      };
      setTransactionHistory(prev => [transaction, ...prev]);

      // Update local balance state
      const currentBalance = balance;
      setBalance({
        ...currentBalance,
        total_balance: currentBalance.total_balance - totalWithdrawAmount,
        locked_balance: currentBalance.locked_balance - withdrawAmountInBigInt,
        available_balance: currentBalance.available_balance - totalWithdrawAmount,
        deposits: currentBalance.deposits.filter(deposit => 
          deposit.amount !== withdrawAmountInBigInt
        )
      });

      setShowWarningModal(false);
      setSelectedDeposit(null);
    } catch (err) {
      console.error('Error withdrawing:', err);
      setError('Failed to withdraw. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    logout();
  };

  const formatAmount = (amount) => {
    if (!amount) return '0';
    return Number(amount).toFixed(2);
  };

  const getInterestRate = (period) => {
    switch (period) {
      case 'ThreeMonths':
        return '5%';
      case 'SixMonths':
        return '8%';
      case 'TwelveMonths':
        return '12%';
      default:
        return '0%';
    }
  };

  const calculatePenalty = (deposit) => {
    return (Number(deposit.amount) * 0.1).toFixed(2);
  };

  const calculateWithdrawAmount = (deposit) => {
    return (Number(deposit.amount) * 0.9).toFixed(2);
  };

  return (
    <div className="dashboard-container">
      <nav className="dashboard-nav">
        <div className="logo">
          <img src="/logo.png" alt="PiggyBank Logo" className="logo-image" />
          <span className="logo-text">PiggyBank</span>
        </div>
        <div className="user-info">
          <span className="principal-id">ID: {principal?.toString()}</span>
          <button onClick={handleLogout} className="logout-button">
            Logout
          </button>
        </div>
      </nav>

      <div className="dashboard-content">
        <div className="dashboard-header">
          <h1>Guard your cash, grow your stash</h1>
          {error && <div className="error-message">{error}</div>}
        </div>

        <div className="balance-section">
          <div className="balance-card">
            <h2>Total Balance</h2>
            <div className="balance-amount">
              {loading ? 'Loading...' : `₹${formatAmount(balance?.total_balance)}`}
            </div>
          </div>
          <div className="balance-card">
            <h2>Available Balance</h2>
            <div className="balance-amount">
              {loading ? 'Loading...' : `₹${formatAmount(balance?.available_balance)}`}
            </div>
          </div>
          <div className="balance-card">
            <h2>Locked Balance</h2>
            <div className="balance-amount">
              {loading ? 'Loading...' : `₹${formatAmount(balance?.locked_balance)}`}
            </div>
          </div>
          <div className="balance-card">
            <h2>Total Rewards</h2>
            <div className="balance-amount">
              {loading ? 'Loading...' : `₹${formatAmount(balance?.rewards_earned)}`}
            </div>
          </div>
        </div>

        <div className="deposit-section">
          <h2>Deposit INR</h2>
          <form onSubmit={handleDeposit} className="deposit-form">
            <input
              type="number"
              value={depositAmount}
              onChange={(e) => setDepositAmount(e.target.value)}
              placeholder="Enter amount in INR"
              className="input-field"
              disabled={loading}
            />
            <select
              value={selectedLockPeriod}
              onChange={(e) => setSelectedLockPeriod(e.target.value)}
              className="select-field"
              disabled={loading}
            >
              <option value="ThreeMonths">3 Months (5% APY)</option>
              <option value="SixMonths">6 Months (8% APY)</option>
              <option value="TwelveMonths">12 Months (12% APY)</option>
            </select>
            <button
              type="submit"
              className="action-button deposit"
              disabled={loading}
            >
              {loading ? 'Processing...' : 'Deposit'}
            </button>
          </form>
        </div>

        <div className="withdraw-section">
          <h2>Withdraw INR</h2>
          <form onSubmit={handleWithdraw} className="withdraw-form">
            <input
              type="number"
              value={withdrawAmount}
              onChange={(e) => setWithdrawAmount(e.target.value)}
              placeholder="Enter amount in INR"
              className="input-field"
              disabled={loading}
            />
            <button
              type="submit"
              className="action-button withdraw"
              disabled={loading}
            >
              {loading ? 'Processing...' : 'Withdraw'}
            </button>
          </form>
          <p className="warning-text">
            Note: Early withdrawal will incur a 10% penalty
          </p>
        </div>

        <div className="deposits-table">
          <h2>Transaction History</h2>
          <table>
            <thead>
              <tr>
                <th>Type</th>
                <th>Amount</th>
                <th>Lock Period</th>
                <th>Interest Rate</th>
                <th>Date</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {transactionHistory.map((transaction, index) => (
                <tr key={`transaction-${index}`}>
                  <td>
                    <span className={`transaction-type ${transaction.type.toLowerCase()}`}>
                      {transaction.type}
                    </span>
                  </td>
                  <td>₹{formatAmount(transaction.amount)}</td>
                  <td>{transaction.lockPeriod}</td>
                  <td>{transaction.interestRate}</td>
                  <td>{transaction.date.toLocaleDateString()}</td>
                  <td>
                    {transaction.isEarlyWithdrawal && (
                      <span className="early-withdrawal-status">
                        Early Withdrawal (10% Penalty: ₹{formatAmount(transaction.penalty)})
                      </span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {showWarningModal && selectedDeposit && (
        <div className="modal-overlay">
          <div className="modal-content">
            <h2>Early Withdrawal Warning</h2>
            <p>You are attempting to withdraw before the lock period ends.</p>
            <div className="warning-details">
              <p>Original Amount: ₹{formatAmount(selectedDeposit.amount)}</p>
              <p>Lock Period: {Object.keys(selectedDeposit.lock_period)[0]}</p>
              <p>Interest Rate: {getInterestRate(Object.keys(selectedDeposit.lock_period)[0])}</p>
              <p className="penalty">Early Withdrawal Penalty (10%): ₹{calculatePenalty(selectedDeposit)}</p>
              <p className="withdraw-amount">Amount After Penalty: ₹{calculateWithdrawAmount(selectedDeposit)}</p>
              <p className="warning-text">Warning: Early withdrawal will result in a 10% penalty of your deposit amount.</p>
            </div>
            <div className="modal-buttons">
              <button onClick={() => setShowWarningModal(false)} className="cancel-btn">
                Cancel
              </button>
              <button onClick={confirmEarlyWithdraw} className="confirm-btn">
                Confirm Early Withdrawal
              </button>
            </div>
          </div>
        </div>
      )}

      {showInsufficientBalanceModal && (
        <div className="modal-overlay">
          <div className="modal-content">
            <h2>Insufficient Balance</h2>
            <div className="warning-details">
              <p>You don't have enough balance to withdraw ₹{withdrawAmount}</p>
              <p>Total Balance: ₹{formatAmount(balance?.total_balance)}</p>
              <p className="warning-text">Please enter an amount less than or equal to your total balance.</p>
            </div>
            <div className="modal-buttons">
              <button onClick={() => setShowInsufficientBalanceModal(false)} className="confirm-btn">
                OK
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard; 