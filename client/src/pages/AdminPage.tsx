import { useState, useEffect } from 'react';
import type { CSSProperties } from 'react';
import { apiService } from '../services/api';
import type { User, AgentMetaListItem, ApiError } from '../types';
import { Layout } from '../components/common/Layout';
import { ErrorModal } from '../components/common/ErrorModal';

export function AdminPage() {
  const [activeTab, setActiveTab] = useState<'users' | 'agent_metas'>('users');
  const [users, setUsers] = useState<User[]>([]);
  const [agentMetas, setAgentMetas] = useState<AgentMetaListItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  // User form states
  const [newUserName, setNewUserName] = useState('');
  const [newUserEmail, setNewUserEmail] = useState('');
  const [newUserPassword, setNewUserPassword] = useState('');

  // Agent Meta form states
  const [newMetaName, setNewMetaName] = useState('');
  const [newMetaDescription, setNewMetaDescription] = useState('');
  const [newMetaCharacterDesign, setNewMetaCharacterDesign] = useState('');
  const [newMetaResponseRequirement, setNewMetaResponseRequirement] = useState('');
  const [newMetaCharacterEmotionSplit, setNewMetaCharacterEmotionSplit] = useState('');
  const [newMetaModel, setNewMetaModel] = useState('');

  useEffect(() => {
    if (activeTab === 'users') {
      loadUsers();
    } else {
      loadAgentMetas();
    }
  }, [activeTab]);

  const loadUsers = async () => {
    try {
      setLoading(true);
      const data = await apiService.listUsers();
      setUsers(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  const loadAgentMetas = async () => {
    try {
      setLoading(true);
      const data = await apiService.listAgentMetas();
      setAgentMetas(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateUser = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      await apiService.createUser(newUserName, newUserEmail, newUserPassword);
      setSuccessMessage('用户创建成功');
      setNewUserName('');
      setNewUserEmail('');
      setNewUserPassword('');
      await loadUsers();
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateAgentMeta = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      await apiService.createAgentMeta({
        name: newMetaName,
        description: newMetaDescription,
        character_design: newMetaCharacterDesign,
        response_requirement: newMetaResponseRequirement,
        character_emotion_split: newMetaCharacterEmotionSplit,
        model: newMetaModel,
      });
      setSuccessMessage('代理元数据创建成功');
      setNewMetaName('');
      setNewMetaDescription('');
      setNewMetaCharacterDesign('');
      setNewMetaResponseRequirement('');
      setNewMetaCharacterEmotionSplit('');
      setNewMetaModel('');
      await loadAgentMetas();
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  // Styles
  const cardStyle: CSSProperties = {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '24px',
    boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)',
  };

  const titleStyle: CSSProperties = {
    fontSize: '24px',
    fontWeight: 'bold',
    color: '#1f2937',
    marginBottom: '24px',
  };

  const tabsStyle: CSSProperties = {
    display: 'flex',
    gap: '8px',
    marginBottom: '24px',
    borderBottom: '2px solid #e5e7eb',
  };

  const tabStyle = (isActive: boolean): CSSProperties => ({
    padding: '12px 24px',
    cursor: 'pointer',
    fontWeight: '600',
    color: isActive ? '#3b82f6' : '#6b7280',
    borderBottom: isActive ? '2px solid #3b82f6' : 'none',
    marginBottom: '-2px',
    transition: 'all 0.2s',
  });

  const formStyle: CSSProperties = {
    backgroundColor: '#f9fafb',
    padding: '20px',
    borderRadius: '8px',
    marginBottom: '24px',
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
    transition: 'border-color 0.2s',
  };

  const textareaStyle: CSSProperties = {
    ...inputStyle,
    minHeight: '100px',
    resize: 'vertical',
  };

  const buttonStyle: CSSProperties = {
    backgroundColor: '#3b82f6',
    color: 'white',
    padding: '10px 20px',
    border: 'none',
    borderRadius: '6px',
    fontSize: '14px',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'background-color 0.2s',
  };

  const tableStyle: CSSProperties = {
    width: '100%',
    borderCollapse: 'collapse',
  };

  const thStyle: CSSProperties = {
    textAlign: 'left',
    padding: '12px',
    backgroundColor: '#f9fafb',
    borderBottom: '2px solid #e5e7eb',
    fontWeight: '600',
    color: '#374151',
  };

  const tdStyle: CSSProperties = {
    padding: '12px',
    borderBottom: '1px solid #e5e7eb',
    color: '#6b7280',
  };

  const successMessageStyle: CSSProperties = {
    backgroundColor: '#d1fae5',
    color: '#065f46',
    padding: '12px',
    borderRadius: '6px',
    marginBottom: '16px',
    fontSize: '14px',
  };

  return (
    <Layout>
      <div style={cardStyle}>
        <h2 style={titleStyle}>管理后台</h2>

        <div style={tabsStyle}>
          <div
            style={tabStyle(activeTab === 'users')}
            onClick={() => setActiveTab('users')}
          >
            用户管理
          </div>
          <div
            style={tabStyle(activeTab === 'agent_metas')}
            onClick={() => setActiveTab('agent_metas')}
          >
            代理元数据管理
          </div>
        </div>

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

        {activeTab === 'users' && (
          <>
            <form onSubmit={handleCreateUser} style={formStyle}>
              <h3 style={formTitleStyle}>创建新用户</h3>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>用户名称</label>
                <input
                  type="text"
                  value={newUserName}
                  onChange={(e) => setNewUserName(e.target.value)}
                  style={inputStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>邮箱</label>
                <input
                  type="email"
                  value={newUserEmail}
                  onChange={(e) => setNewUserEmail(e.target.value)}
                  style={inputStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>密码</label>
                <input
                  type="password"
                  value={newUserPassword}
                  onChange={(e) => setNewUserPassword(e.target.value)}
                  style={inputStyle}
                  required
                />
              </div>
              <button
                type="submit"
                style={buttonStyle}
                disabled={loading}
                onMouseEnter={(e) => {
                  if (!loading) e.currentTarget.style.backgroundColor = '#2563eb';
                }}
                onMouseLeave={(e) => {
                  if (!loading) e.currentTarget.style.backgroundColor = '#3b82f6';
                }}
              >
                {loading ? '创建中...' : '创建用户'}
              </button>
            </form>

            <h3 style={{ fontSize: '18px', fontWeight: '600', marginBottom: '16px' }}>
              用户列表
            </h3>
            {loading ? (
              <div style={{ textAlign: 'center', padding: '20px', color: '#6b7280' }}>
                加载中...
              </div>
            ) : users.length === 0 ? (
              <div style={{ textAlign: 'center', padding: '20px', color: '#9ca3af' }}>
                暂无用户
              </div>
            ) : (
              <table style={tableStyle}>
                <thead>
                  <tr>
                    <th style={thStyle}>ID</th>
                    <th style={thStyle}>名称</th>
                    <th style={thStyle}>邮箱</th>
                  </tr>
                </thead>
                <tbody>
                  {users.map((user) => (
                    <tr key={user.id}>
                      <td style={tdStyle}>{user.id}</td>
                      <td style={tdStyle}>{user.name}</td>
                      <td style={tdStyle}>{user.email}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </>
        )}

        {activeTab === 'agent_metas' && (
          <>
            <form onSubmit={handleCreateAgentMeta} style={formStyle}>
              <h3 style={formTitleStyle}>创建新代理元数据</h3>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>名称</label>
                <input
                  type="text"
                  value={newMetaName}
                  onChange={(e) => setNewMetaName(e.target.value)}
                  style={inputStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>描述</label>
                <textarea
                  value={newMetaDescription}
                  onChange={(e) => setNewMetaDescription(e.target.value)}
                  style={textareaStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>角色设计</label>
                <textarea
                  value={newMetaCharacterDesign}
                  onChange={(e) => setNewMetaCharacterDesign(e.target.value)}
                  style={textareaStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>回复要求</label>
                <textarea
                  value={newMetaResponseRequirement}
                  onChange={(e) => setNewMetaResponseRequirement(e.target.value)}
                  style={textareaStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>角色情绪分割</label>
                <textarea
                  value={newMetaCharacterEmotionSplit}
                  onChange={(e) => setNewMetaCharacterEmotionSplit(e.target.value)}
                  style={textareaStyle}
                  required
                />
              </div>
              <div style={inputGroupStyle}>
                <label style={labelStyle}>AI 模型</label>
                <input
                  type="text"
                  value={newMetaModel}
                  onChange={(e) => setNewMetaModel(e.target.value)}
                  style={inputStyle}
                  required
                />
              </div>
              <button
                type="submit"
                style={buttonStyle}
                disabled={loading}
                onMouseEnter={(e) => {
                  if (!loading) e.currentTarget.style.backgroundColor = '#2563eb';
                }}
                onMouseLeave={(e) => {
                  if (!loading) e.currentTarget.style.backgroundColor = '#3b82f6';
                }}
              >
                {loading ? '创建中...' : '创建代理元数据'}
              </button>
            </form>

            <h3 style={{ fontSize: '18px', fontWeight: '600', marginBottom: '16px' }}>
              代理元数据列表
            </h3>
            {loading ? (
              <div style={{ textAlign: 'center', padding: '20px', color: '#6b7280' }}>
                加载中...
              </div>
            ) : agentMetas.length === 0 ? (
              <div style={{ textAlign: 'center', padding: '20px', color: '#9ca3af' }}>
                暂无代理元数据
              </div>
            ) : (
              <table style={tableStyle}>
                <thead>
                  <tr>
                    <th style={thStyle}>ID</th>
                    <th style={thStyle}>名称</th>
                    <th style={thStyle}>描述</th>
                  </tr>
                </thead>
                <tbody>
                  {agentMetas.map((meta) => (
                    <tr key={meta.id}>
                      <td style={tdStyle}>{meta.id}</td>
                      <td style={tdStyle}>{meta.name}</td>
                      <td style={tdStyle}>{meta.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </>
        )}
      </div>
      {error && <ErrorModal message={error} onClose={() => setError(null)} />}
    </Layout>
  );
}
