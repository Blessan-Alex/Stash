import React, { createContext, useState, useContext, useEffect } from 'react';
import { AuthClient } from '@dfinity/auth-client';
import { Principal } from '@dfinity/principal';

const AuthContext = createContext();

export const useAuth = () => {
  return useContext(AuthContext);
};

export const AuthProvider = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [principal, setPrincipal] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    checkAuth();
  }, []);

  const checkAuth = async () => {
    try {
      const client = await AuthClient.create();
      const isAuthed = await client.isAuthenticated();
      if (isAuthed) {
        const identity = client.getIdentity();
        setPrincipal(identity.getPrincipal());
        setIsAuthenticated(true);
      }
    } catch (error) {
      console.error('Auth check failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const login = async () => {
    try {
      const client = await AuthClient.create({
        identityProvider: 'https://identity.ic0.app',
      });
      await client.login({
        identityProvider: 'https://identity.ic0.app',
        onSuccess: () => {
          const identity = client.getIdentity();
          setPrincipal(identity.getPrincipal());
          setIsAuthenticated(true);
        },
      });
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    }
  };

  const logout = async () => {
    try {
      const client = await AuthClient.create();
      await client.logout();
      setPrincipal(null);
      setIsAuthenticated(false);
    } catch (error) {
      console.error('Logout failed:', error);
      throw error;
    }
  };

  const value = {
    isAuthenticated,
    principal,
    loading,
    login,
    logout,
  };

  return (
    <AuthContext.Provider value={value}>
      {!loading && children}
    </AuthContext.Provider>
  );
}; 