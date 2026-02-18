import { createContext, useContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';
import { apiService } from '../services/api';
import { SESSION_TOKEN_KEY } from '../utils/constants';

interface AuthContextType {
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  loading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Check if user has a session token in localStorage
    const validateToken = async () => {
      const token = localStorage.getItem(SESSION_TOKEN_KEY);
      if (token) {
        try {
          // Try to make an authenticated request to validate the token
          await apiService.listAgents();
          setIsAuthenticated(true);
        } catch (err) {
          // Token is invalid, remove it
          // Log non-authentication errors for debugging
          if (typeof err === 'object' && err !== null && 'status' in err) {
            const apiError = err as { status: number };
            if (apiError.status !== 401 && apiError.status !== 403) {
              console.error('Token validation failed with unexpected error:', err);
            }
          }
          localStorage.removeItem(SESSION_TOKEN_KEY);
          setIsAuthenticated(false);
        }
      } else {
        setIsAuthenticated(false);
      }
      setLoading(false);
    };

    validateToken();
  }, []);

  const login = async (email: string, password: string) => {
    const token = await apiService.login(email, password);
    localStorage.setItem(SESSION_TOKEN_KEY, token);
    setIsAuthenticated(true);
  };

  const logout = async () => {
    try {
      await apiService.logout();
    } finally {
      localStorage.removeItem(SESSION_TOKEN_KEY);
      setIsAuthenticated(false);
    }
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated, login, logout, loading }}>
      {children}
    </AuthContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
