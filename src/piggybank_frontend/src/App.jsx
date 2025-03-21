import React from 'react';
import ReactTypingEffect from 'react-typing-effect';
import './App.css';

function App() {
  return (
    <div className="app-container">
      <nav className="navbar">
        <div className="logo">
          <img src="/logo.png" alt="Stash Logo" className="logo-image" />
          <span className="logo-text">Stash</span>
        </div>
      </nav>

      <main className="hero-section">
        <div className="hero-content">
          <h1>Welcome to Stash</h1>
          <p className="subtitle">
            <ReactTypingEffect
              text={["Your Piggybank on STEROIDS!!"]}
              speed={50}
              eraseDelay={700000}
              displayTextRenderer={(text, i) => {
                return (
                  <span>
                    {text}
                  </span>
                );
              }}
            />
          </p>
          <button className="start-button">
            Start Now
            <span className="arrow">→</span>
          </button>
          <div className="features">
            <div className="feature">
              <span className="feature-icon">💰</span>
              <h3>Save Securely</h3>
              <p>Store your savings with blockchain security</p>
            </div>
            <div className="feature">
              <span className="feature-icon">📈</span>
              <h3>Earn Rewards</h3>
              <p>Grow your savings with interest</p>
            </div>
            <div className="feature">
              <span className="feature-icon">🔒</span>
              <h3>Protected</h3>
              <p>Your funds are always safe and secure</p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
