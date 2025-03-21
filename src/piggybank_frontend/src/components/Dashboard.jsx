import React, { useState, useEffect } from 'react';
import { useAuth } from '../context/AuthContext';
import { useNavigate } from 'react-router-dom';
import './Dashboard.css';

const Dashboard = () => {
  const { principal, logout } = useAuth();
  const navigate = useNavigate();
  const [balance, setBalance] = useState(null);
  const [depositAmount, setDepositAmount] = useState('');
  const [withdrawAmount, setWithdrawAmount] = useState('');
  const [selectedLockPeriod, setSelectedLockPeriod] = useState('ThreeMonths');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleLogout = async () => {
    try {
      await logout();
      navigate('/');
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  const fetchBalance = async () => {
    try {
      setLoading(true);
      // TODO: Implement get_balance call to backend
      // const response = await window.canister.piggybank_backend.get_balance();
      // setBalance(response);
    } catch (error) {
      setError('Failed to fetch balance');
      console.error('Balance fetch failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDeposit = async () => {
    try {
      setLoading(true);
      setError('');
      // TODO: Implement mint_tokens call to backend
      // const response = await window.canister.piggybank_backend.mint_tokens(
      //   BigInt(depositAmount),
      //   selectedLockPeriod
      // );
      // if (response.Ok) {
      //   await fetchBalance();
      //   setDepositAmount('');
      // } else {
      //   setError(response.Err);
      // }
    } catch (error) {
      setError('Deposit failed');
      console.error('Deposit failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleWithdraw = async () => {
    try {
      setLoading(true);
      setError('');
      // TODO: Implement burn_tokens call to backend
      // const response = await window.canister.piggybank_backend.burn_tokens(
      //   BigInt(withdrawAmount)
      // );
      // if (response.Ok) {
      //   await fetchBalance();
      //   setWithdrawAmount('');
      // } else {
      //   setError(response.Err);
      // }
    } catch (error) {
      setError('Withdrawal failed');
      console.error('Withdrawal failed:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBalance();
  }, []);

  const formatAmount = (amount) => {
    return (Number(amount) / 1e8).toFixed(2);
  };

  const getInterestRate = (period) => {
    switch (period) {
      case 'ThreeMonths': return '5%';
      case 'SixMonths': return '7%';
      case 'TwelveMonths': return '10%';
      default: return '0%';
    }
  };

  return (
    <div className="dashboard-container">
      <nav className="dashboard-nav">
        <div className="logo">
          <img src="/logo.png" alt="Stash Logo" className="logo-image" />
          <span className="logo-text">Stash</span>
        </div>
        <div className="user-info">
          <span className="principal-id">
            Principal ID: {principal?.toString()}
          </span>
          <button onClick={handleLogout} className="logout-button">
            Logout
          </button>
        </div>
      </nav>

      <main className="dashboard-content">
        <div className="dashboard-header">
          <h1>Your Dashboard</h1>
          {error && <div className="error-message">{error}</div>}
        </div>

        <div className="balance-section">
          <div className="balance-card">
            <h2>Total Balance</h2>
            <p className="balance">{loading ? 'Loading...' : formatAmount(balance?.total_balance || 0)} PIGGYUSD</p>
          </div>
          <div className="balance-card">
            <h2>Available Balance</h2>
            <p className="balance">{loading ? 'Loading...' : formatAmount(balance?.available_balance || 0)} PIGGYUSD</p>
          </div>
          <div className="balance-card">
            <h2>Locked Balance</h2>
            <p className="balance">{loading ? 'Loading...' : formatAmount(balance?.locked_balance || 0)} PIGGYUSD</p>
          </div>
          <div className="balance-card">
            <h2>Total Rewards</h2>
            <p className="balance">{loading ? 'Loading...' : formatAmount(balance?.rewards_earned || 0)} PIGGYUSD</p>
          </div>
        </div>

        <div className="deposit-section">
          <h2>Deposit</h2>
          <div className="deposit-form">
            <input
              type="number"
              value={depositAmount}
              onChange={(e) => setDepositAmount(e.target.value)}
              placeholder="Enter amount in INR"
              className="input-field"
            />
            <select
              value={selectedLockPeriod}
              onChange={(e) => setSelectedLockPeriod(e.target.value)}
              className="select-field"
            >
              <option value="ThreeMonths">3 Months (5% APY)</option>
              <option value="SixMonths">6 Months (7% APY)</option>
              <option value="TwelveMonths">12 Months (10% APY)</option>
            </select>
            <button
              onClick={handleDeposit}
              disabled={loading || !depositAmount}
              className="action-button deposit"
            >
              {loading ? 'Processing...' : 'Deposit'}
            </button>
          </div>
        </div>

        <div className="withdraw-section">
          <h2>Withdraw</h2>
          <div className="withdraw-form">
            <input
              type="number"
              value={withdrawAmount}
              onChange={(e) => setWithdrawAmount(e.target.value)}
              placeholder="Enter amount in PIGGYUSD"
              className="input-field"
            />
            <button
              onClick={handleWithdraw}
              disabled={loading || !withdrawAmount}
              className="action-button withdraw"
            >
              {loading ? 'Processing...' : 'Withdraw'}
            </button>
          </div>
        </div>

        <div className="deposits-table">
          <h2>Active Deposits</h2>
          <table>
            <thead>
              <tr>
                <th>Amount</th>
                <th>Lock Period</th>
                <th>Interest Rate</th>
                <th>Deposit Date</th>
                <th>Days Remaining</th>
              </tr>
            </thead>
            <tbody>
              {balance?.deposits.map((deposit, index) => {
                const depositDate = new Date(Number(deposit.deposit_time) / 1_000_000);
                const lockEndDate = new Date(Number(deposit.deposit_time + deposit.lock_period.duration_nanos()) / 1_000_000);
                const daysRemaining = Math.ceil((lockEndDate - new Date()) / (1000 * 60 * 60 * 24));
                
                return (
                  <tr key={index}>
                    <td>{formatAmount(deposit.amount)} PIGGYUSD</td>
                    <td>{deposit.lock_period}</td>
                    <td>{(deposit.interest_rate * 100).toFixed(1)}%</td>
                    <td>{depositDate.toLocaleDateString()}</td>
                    <td>{daysRemaining} days</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
};

export default Dashboard; 