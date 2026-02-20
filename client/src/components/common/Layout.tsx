import type {ReactNode, CSSProperties} from 'react';
import {useNavigate} from 'react-router-dom';
import {useAuth} from '../../contexts/AuthContext';

interface LayoutProps {
    children: ReactNode;
}

export function Layout({children}: LayoutProps) {
    const {logout} = useAuth();
    const navigate = useNavigate();

    const handleLogout = async () => {
        await logout();
        navigate('/login');
    };

    const containerStyle: CSSProperties = {
        minHeight: '100vh',
        backgroundColor: '#f3f4f6',
    };

    const titleStyle: CSSProperties = {
        fontSize: '24px',
        fontWeight: 'bold',
        color: '#1f2937',
        cursor: 'pointer',
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

    return (
        <div style={containerStyle}>
            <header className="layout-header">
                <h1 style={titleStyle} onClick={() => navigate('/')}>
                    RPG Stage
                </h1>
                <nav className="layout-nav">
          <span style={linkStyle} onClick={() => navigate('/')}>
            Agent列表
          </span>
                    <span style={linkStyle} onClick={() => navigate('/profile')}>
            个人资料
          </span>
                    {/* <span style={linkStyle} onClick={() => navigate('/admin')}>
            管理后台
          </span> */}
                    <button style={buttonStyle} onClick={handleLogout}>
                        退出登录
                    </button>
                </nav>
            </header>
            <main className="layout-content">{children}</main>
        </div>
    );
}
