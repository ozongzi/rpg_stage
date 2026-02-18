import type { ReactNode, CSSProperties } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../contexts/AuthContext';

interface LayoutProps {
  children: ReactNode;
}

export function Layout({ children }: LayoutProps) {
  const { logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    navigate('/login');
  };

  const containerStyle: CSSProperties = {
    minHeight: '100vh',
    backgroundColor: '#f3f4f6',
  };

  const headerStyle: CSSProperties = {
    backgroundColor: 'white',
    borderBottom: '1px solid #e5e7eb',
    padding: '16px 24px',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)',
  };

  const titleStyle: CSSProperties = {
    fontSize: '24px',
    fontWeight: 'bold',
    color: '#1f2937',
    cursor: 'pointer',
  };

  const navStyle: CSSProperties = {
    display: 'flex',
    gap: '16px',
    alignItems: 'center',
  };

  const linkStyle: CSSProperties = {
    color: '#4b5563',
    textDecoration: 'none',
    padding: '8px 16px',
    borderRadius: '6px',
    cursor: 'pointer',
    transition: 'background-color 0.2s',
  };

  const buttonStyle: CSSProperties = {
    ...linkStyle,
    backgroundColor: '#ef4444',
    color: 'white',
    border: 'none',
    fontWeight: '500',
  };

  const contentStyle: CSSProperties = {
    padding: '24px',
    maxWidth: '1400px',
    margin: '0 auto',
  };

  return (
    <div style={containerStyle}>
      <header style={headerStyle}>
        <h1 style={titleStyle} onClick={() => navigate('/')}>
          RPG Stage
        </h1>
        <nav style={navStyle}>
          <span style={linkStyle} onClick={() => navigate('/')}>
            代理列表
          </span>
          <span style={linkStyle} onClick={() => navigate('/admin')}>
            管理员
          </span>
          <button style={buttonStyle} onClick={handleLogout}>
            退出登录
          </button>
        </nav>
      </header>
      <main style={contentStyle}>{children}</main>
    </div>
  );
}
