import {useState, useEffect} from 'react';
import type {CSSProperties, FormEvent} from 'react';
import {apiService} from '../services/api';
import type {User, ApiError} from '../types';
import {Layout} from '../components/common/Layout';
import {ErrorModal} from '../components/common/ErrorModal';

export function ProfilePage() {
    const [user, setUser] = useState<User | null>(null);
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [successMessage, setSuccessMessage] = useState<string | null>(null);

    const [oldPassword, setOldPassword] = useState('');
    const [newName, setNewName] = useState('');
    const [newEmail, setNewEmail] = useState('');
    const [newPassword, setNewPassword] = useState('');

    useEffect(() => {
        loadProfile();
    }, []);

    const loadProfile = async () => {
        try {
            setLoading(true);
            const data = await apiService.getCurrentUser();
            setUser(data);
            setNewName(data.name || '');
            setNewEmail(data.email || '');
        } catch (err) {
            const apiError = err as ApiError;
            setError(apiError.message);
        } finally {
            setLoading(false);
        }
    };

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();
        try {
            setSaving(true);
            const updates: { old_password: string; name?: string; email?: string; password?: string } = {
                old_password: oldPassword,
            };
            if (newName && user && newName !== user.name) updates.name = newName;
            if (newEmail && user && newEmail !== user.email) updates.email = newEmail;
            if (newPassword) updates.password = newPassword;

            const updated = await apiService.updateCurrentUser(updates);
            setUser(updated);
            setSuccessMessage('个人信息更新成功');
            setOldPassword('');
            setNewPassword('');
        } catch (err) {
            const apiError = err as ApiError;
            setError(apiError.message);
        } finally {
            setSaving(false);
        }
    };

    const cardStyle: CSSProperties = {
        backgroundColor: 'white',
        borderRadius: '8px',
        padding: '24px',
        boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)',
        maxWidth: '600px',
        margin: '0 auto',
    };

    const titleStyle: CSSProperties = {
        fontSize: '24px',
        fontWeight: 'bold',
        color: '#1f2937',
        marginBottom: '24px',
    };

    const infoBoxStyle: CSSProperties = {
        backgroundColor: '#f9fafb',
        borderRadius: '8px',
        padding: '16px',
        marginBottom: '24px',
    };

    const infoRowStyle: CSSProperties = {
        display: 'flex',
        justifyContent: 'space-between',
        marginBottom: '8px',
        fontSize: '14px',
    };

    const formStyle: CSSProperties = {
        borderTop: '1px solid #e5e7eb',
        paddingTop: '24px',
    };

    const formTitleStyle: CSSProperties = {
        fontSize: '18px',
        fontWeight: '600',
        color: '#1f2937',
        marginBottom: '16px',
    };

    const inputGroupStyle: CSSProperties = {
        marginBottom: '16px',
    };

    const labelStyle: CSSProperties = {
        display: 'block',
        fontSize: '14px',
        fontWeight: '500',
        color: '#374151',
        marginBottom: '6px',
    };

    const inputStyle: CSSProperties = {
        width: '100%',
        padding: '10px',
        border: '1px solid #d1d5db',
        borderRadius: '6px',
        fontSize: '14px',
        outline: 'none',
        boxSizing: 'border-box',
    };

    const buttonStyle: CSSProperties = {
        backgroundColor: '#3b82f6',
        color: 'white',
        padding: '10px 20px',
        border: 'none',
        borderRadius: '6px',
        fontSize: '14px',
        fontWeight: '600',
        cursor: saving ? 'not-allowed' : 'pointer',
        transition: 'background-color 0.2s',
    };

    const successMessageStyle: CSSProperties = {
        backgroundColor: '#d1fae5',
        color: '#065f46',
        padding: '12px',
        borderRadius: '6px',
        marginBottom: '16px',
        fontSize: '14px',
    };

    if (loading) {
        return (
            <Layout>
                <div style={{textAlign: 'center', padding: '40px', color: '#6b7280'}}>加载中...</div>
            </Layout>
        );
    }

    return (
        <Layout>
            <div style={cardStyle}>
                <h2 style={titleStyle}>个人资料</h2>

                {successMessage && (
                    <div style={successMessageStyle}>
                        {successMessage}
                        <button
                            onClick={() => setSuccessMessage(null)}
                            aria-label="关闭消息"
                            style={{
                                float: 'right',
                                background: 'none',
                                border: 'none',
                                cursor: 'pointer',
                                fontSize: '16px',
                                fontWeight: 'bold',
                                color: '#065f46',
                            }}
                        >
                            ✕
                        </button>
                    </div>
                )}

                {user && (
                    <div style={infoBoxStyle}>
                        <div style={infoRowStyle}>
                            <span style={{color: '#6b7280'}}>用户 ID:</span>
                            <span style={{color: '#1f2937'}}>{user.id}</span>
                        </div>
                        <div style={infoRowStyle}>
                            <span style={{color: '#6b7280'}}>用户名:</span>
                            <span style={{color: '#1f2937'}}>{user.name}</span>
                        </div>
                        <div style={infoRowStyle}>
                            <span style={{color: '#6b7280'}}>邮箱:</span>
                            <span style={{color: '#1f2937'}}>{user.email}</span>
                        </div>
                    </div>
                )}

                <div style={formStyle}>
                    <h3 style={formTitleStyle}>修改信息</h3>
                    <form onSubmit={handleSubmit}>
                        <div style={inputGroupStyle}>
                            <label style={labelStyle}>当前密码（必填）</label>
                            <input
                                type="password"
                                value={oldPassword}
                                onChange={(e) => setOldPassword(e.target.value)}
                                style={inputStyle}
                                required
                            />
                        </div>
                        <div style={inputGroupStyle}>
                            <label style={labelStyle}>新用户名（可选）</label>
                            <input
                                type="text"
                                value={newName}
                                onChange={(e) => setNewName(e.target.value)}
                                style={inputStyle}
                            />
                        </div>
                        <div style={inputGroupStyle}>
                            <label style={labelStyle}>新邮箱（可选）</label>
                            <input
                                type="email"
                                value={newEmail}
                                onChange={(e) => setNewEmail(e.target.value)}
                                style={inputStyle}
                            />
                        </div>
                        <div style={inputGroupStyle}>
                            <label style={labelStyle}>新密码（可选）</label>
                            <input
                                type="password"
                                value={newPassword}
                                onChange={(e) => setNewPassword(e.target.value)}
                                style={inputStyle}
                            />
                        </div>
                        <button
                            type="submit"
                            style={buttonStyle}
                            disabled={saving}
                            onMouseEnter={(e) => {
                                if (!saving) e.currentTarget.style.backgroundColor = '#2563eb';
                            }}
                            onMouseLeave={(e) => {
                                if (!saving) e.currentTarget.style.backgroundColor = '#3b82f6';
                            }}
                        >
                            {saving ? '保存中...' : '保存修改'}
                        </button>
                    </form>
                </div>
            </div>
            {error && <ErrorModal message={error} onClose={() => setError(null)}/>}
        </Layout>
    );
}
