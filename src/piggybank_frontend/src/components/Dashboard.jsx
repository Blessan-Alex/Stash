import React from 'react';
import { useAuth } from './auth/AuthProvider';
import './Dashboard.css';

export function Dashboard() {
  const { logout } = useAuth();

  return (
    <div className="dashboard">
      <nav className="dashboard-nav">
        <div className="logo">PiggyBank</div>
        <button className="logout-button" onClick={logout}>
          Logout
        </button>
      </nav>

      <main className="dashboard-content">
        <div className="dashboard-header">
          <h1>Your Dashboard</h1>
          <p>Welcome to your savings dashboard</p>
        </div>

        <div className="dashboard-grid">
          <div className="dashboard-card">
            <h3>Total Savings</h3>
            <p className="amount">₹0.00</p>
          </div>
          <div className="dashboard-card">
            <h3>Rewards</h3>
            <p className="amount">₹0.00</p>
          </div>
          <div className="dashboard-card">
            <h3>Interest Rate</h3>
            <p className="amount">5%</p>
          </div>
        </div>

        <div className="dashboard-actions">
          <button className="action-button deposit">Deposit</button>
          <button className="action-button withdraw">Withdraw</button>
        </div>
      </main>
    </div>
  );
} 