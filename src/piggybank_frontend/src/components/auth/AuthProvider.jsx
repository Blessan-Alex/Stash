import React, { createContext, useContext, useState, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';

const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    initAuth();
  }, []);

  async function initAuth() {
    try {
      const client = await AuthClient.create();
      const isAuthed = await client.isAuthenticated();
      setIsAuthenticated(isAuthed);
    } catch (error) {
      console.error('Auth initialization error:', error);
    } finally {
      setIsLoading(false);
    }
  }

  async function login() {
    try {
      const client = await AuthClient.create();
      await client.login({
        identityProvider: 'http://be2us-64aaa-aaaaa-qaabq-cai.localhost:4943',
        onSuccess: () => {
          setIsAuthenticated(true);
        },
      });
    } catch (error) {
      console.error('Login error:', error);
    }
  }

  async function logout() {
    try {
      const client = await AuthClient.create();
      await client.logout();
      setIsAuthenticated(false);
    } catch (error) {
      console.error('Logout error:', error);
    }
  }

  return (
    <AuthContext.Provider value={{ isAuthenticated, isLoading, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
} 