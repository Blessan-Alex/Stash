import React from 'react';
import ReactTypingEffect from 'react-typing-effect';
import { BrowserRouter as Router, Routes, Route, Navigate, useNavigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './context/AuthContext';
import Dashboard from './components/Dashboard';
import './App.css';

const LandingPage = () => {
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleStart = async () => {
    try {
      await login();
      navigate('/dashboard');
    } catch (error) {
      console.error('Login failed:', error);
    }
  };

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
              text={[
                "Your Piggybank on STEROIDS!!",
                "Save Smarter, Earn More",
                "Secure Your Future",
                "Your Money, Your Control"
              ]}
              speed={50}
              eraseDelay={2000}
              typingDelay={1000}
              displayTextRenderer={(text, i) => {
                return (
                  <span className="typing-text">
                    {text}
                  </span>
                );
              }}
            />
          </p>
          <button className="start-button" onClick={handleStart}>
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
};

const PrivateRoute = ({ children }) => {
  const { isAuthenticated } = useAuth();
  return isAuthenticated ? children : <Navigate to="/" />;
};

function App() {
  return (
    <Router>
      <AuthProvider>
        <Routes>
          <Route path="/" element={<LandingPage />} />
          <Route
            path="/dashboard"
            element={
              <PrivateRoute>
                <Dashboard />
              </PrivateRoute>
            }
          />
        </Routes>
      </AuthProvider>
    </Router>
  );
}

export default App;
