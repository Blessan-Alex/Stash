import React, { useState, useEffect } from 'react';
import { canister } from '../declarations/piggybank_backend';

function TokenManager() {
  const [balance, setBalance] = useState(null);
  const [amount, setAmount] = useState('');
  const [lockPeriod, setLockPeriod] = useState('ThreeMonths');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  useEffect(() => {
    fetchBalance();
  }, []);

  const fetchBalance = async () => {
    try {
      const result = await canister.get_balance();
      if (result.Ok) {
        setBalance(result.Ok);
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError('Failed to fetch balance');
    }
  };

  const handleMint = async () => {
    try {
      const inrAmount = parseInt(amount);
      if (isNaN(inrAmount) || inrAmount <= 0) {
        setError('Please enter a valid amount');
        return;
      }

      const result = await canister.mint_tokens(inrAmount, { [lockPeriod]: null });
      if (result.Ok) {
        setSuccess(`Successfully minted ${result.Ok} tokens`);
        fetchBalance();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError('Failed to mint tokens');
    }
  };

  const handleBurn = async () => {
    try {
      const tokenAmount = parseInt(amount);
      if (isNaN(tokenAmount) || tokenAmount <= 0) {
        setError('Please enter a valid amount');
        return;
      }

      const result = await canister.burn_tokens(tokenAmount);
      if (result.Ok) {
        setSuccess(`Successfully burned ${result.Ok} tokens`);
        fetchBalance();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError('Failed to burn tokens');
    }
  };

  const handleApplyRewards = async () => {
    try {
      const result = await canister.apply_rewards();
      if (result.Ok) {
        setSuccess(`Successfully applied ${result.Ok} rewards`);
        fetchBalance();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError('Failed to apply rewards');
    }
  };

  return (
    <div className="token-manager">
      <h2>Token Management</h2>
      
      {error && <div className="error">{error}</div>}
      {success && <div className="success">{success}</div>}

      {balance && (
        <div className="balance-info">
          <h3>Your Balance</h3>
          <p>Total Balance: {balance.total_balance}</p>
          <p>Locked Balance: {balance.locked_balance}</p>
          <p>Available Balance: {balance.available_balance}</p>
          <p>Rewards Earned: {balance.rewards_earned}</p>
        </div>
      )}

      <div className="token-actions">
        <div className="input-group">
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Enter amount"
          />
          <select value={lockPeriod} onChange={(e) => setLockPeriod(e.target.value)}>
            <option value="ThreeMonths">3 Months (5% APY)</option>
            <option value="SixMonths">6 Months (7% APY)</option>
            <option value="TwelveMonths">12 Months (10% APY)</option>
          </select>
        </div>

        <div className="button-group">
          <button onClick={handleMint}>Mint Tokens</button>
          <button onClick={handleBurn}>Burn Tokens</button>
          <button onClick={handleApplyRewards}>Apply Rewards</button>
        </div>
      </div>
    </div>
  );
}

export default TokenManager; 