import { useState, useEffect } from 'react';
import type { CSSProperties } from 'react';
import { useNavigate } from 'react-router-dom';
import { apiService } from '../services/api';
import type { Agent, AgentMetaListItem, ApiError } from '../types';
import { Layout } from '../components/common/Layout';
import { ErrorModal } from '../components/common/ErrorModal';

export function AgentsPage() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [agentMetas, setAgentMetas] = useState<AgentMetaListItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [selectedMetaId, setSelectedMetaId] = useState<string>('');
  const navigate = useNavigate();

  useEffect(() => {
    loadAgents();
    loadAgentMetas();
  }, []);

  const loadAgents = async () => {
    try {
      setLoading(true);
      const data = await apiService.listAgents();
      setAgents(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  const loadAgentMetas = async () => {
    try {
      const data = await apiService.listAgentMetas();
      setAgentMetas(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    }
  };

  const handleCreateAgent = async () => {
    try {
      setLoading(true);
      await apiService.createAgent(selectedMetaId);
      setShowCreateModal(false);
      setSelectedMetaId('');
      await loadAgents();
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    } finally {
      setLoading(false);
    }
  };

  const handleAgentClick = (agentId: string) => {
    navigate(`/agents/${agentId}`);
  };

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

  const gridStyle: CSSProperties = {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))',
    gap: '20px',
  };

  const agentCardStyle: CSSProperties = {
    backgroundColor: 'white',
    border: '1px solid #e5e7eb',
    borderRadius: '8px',
    padding: '20px',
    cursor: 'pointer',
    transition: 'all 0.2s',
    boxShadow: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
  };

  const agentNameStyle: CSSProperties = {
    fontSize: '18px',
    fontWeight: '600',
    color: '#1f2937',
    marginBottom: '12px',
  };

  const agentInfoStyle: CSSProperties = {
    display: 'flex',
    justifyContent: 'space-between',
    marginBottom: '8px',
    fontSize: '14px',
    color: '#6b7280',
  };

  const emotionStyle: CSSProperties = {
    display: 'inline-block',
    padding: '4px 12px',
    backgroundColor: '#dbeafe',
    color: '#1e40af',
    borderRadius: '12px',
    fontSize: '13px',
    fontWeight: '500',
  };

  const loadingStyle: CSSProperties = {
    textAlign: 'center',
    padding: '40px',
    color: '#6b7280',
  };

  const emptyStyle: CSSProperties = {
    textAlign: 'center',
    padding: '40px',
    color: '#9ca3af',
  };

  const createButtonStyle: CSSProperties = {
    backgroundColor: '#3b82f6',
    color: 'white',
    padding: '12px 24px',
    border: 'none',
    borderRadius: '6px',
    fontSize: '16px',
    fontWeight: '600',
    cursor: 'pointer',
    marginBottom: '20px',
    transition: 'background-color 0.2s',
  };

  const modalOverlayStyle: CSSProperties = {
    position: 'fixed',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    zIndex: 1000,
  };

  const modalStyle: CSSProperties = {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '24px',
    maxWidth: '500px',
    width: '90%',
    maxHeight: '80vh',
    overflow: 'auto',
  };

  const modalTitleStyle: CSSProperties = {
    fontSize: '20px',
    fontWeight: 'bold',
    color: '#1f2937',
    marginBottom: '20px',
  };

  const selectStyle: CSSProperties = {
    width: '100%',
    padding: '10px',
    border: '1px solid #d1d5db',
    borderRadius: '6px',
    fontSize: '14px',
    marginBottom: '20px',
    outline: 'none',
  };

  const modalButtonsStyle: CSSProperties = {
    display: 'flex',
    gap: '12px',
    justifyContent: 'flex-end',
  };

  const buttonPrimaryStyle: CSSProperties = {
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

  const buttonSecondaryStyle: CSSProperties = {
    backgroundColor: '#e5e7eb',
    color: '#374151',
    padding: '10px 20px',
    border: 'none',
    borderRadius: '6px',
    fontSize: '14px',
    fontWeight: '600',
    cursor: 'pointer',
    transition: 'background-color 0.2s',
  };

  return (
    <Layout>
      <div style={cardStyle}>
        <h2 style={titleStyle}>我的代理</h2>
        <button
          style={createButtonStyle}
          onClick={() => setShowCreateModal(true)}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = '#2563eb';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = '#3b82f6';
          }}
        >
          ➕ 创建新代理
        </button>
        {loading ? (
          <div style={loadingStyle}>加载中...</div>
        ) : agents.length === 0 ? (
          <div style={emptyStyle}>暂无代理，请先创建代理</div>
        ) : (
          <div style={gridStyle}>
            {agents.map((agent) => (
              <div
                key={agent.id}
                style={agentCardStyle}
                onClick={() => handleAgentClick(agent.id)}
                onMouseEnter={(e) => {
                  e.currentTarget.style.transform = 'translateY(-2px)';
                  e.currentTarget.style.boxShadow = '0 4px 6px -1px rgba(0, 0, 0, 0.1)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = 'translateY(0)';
                  e.currentTarget.style.boxShadow = '0 1px 2px 0 rgba(0, 0, 0, 0.05)';
                }}
              >
                <div style={agentNameStyle}>{agent.name}</div>
                <div style={agentInfoStyle}>
                  <span>情绪:</span>
                  <span style={emotionStyle}>{agent.emotion}</span>
                </div>
                <div style={agentInfoStyle}>
                  <span>好感度:</span>
                  <span style={{ fontWeight: '600', color: '#059669' }}>{agent.favorability}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {showCreateModal && (
        <div style={modalOverlayStyle} onClick={() => setShowCreateModal(false)}>
          <div style={modalStyle} onClick={(e) => e.stopPropagation()}>
            <h3 style={modalTitleStyle}>创建新代理</h3>
            <label style={{ display: 'block', marginBottom: '8px', fontSize: '14px', fontWeight: '500', color: '#374151' }}>
              选择代理元数据
            </label>
            <select
              value={selectedMetaId}
              onChange={(e) => setSelectedMetaId(e.target.value)}
              style={selectStyle}
            >
              <option value="">-- 请选择 --</option>
              {agentMetas.map((meta) => (
                <option key={meta.id} value={meta.id}>
                  {meta.name} - {meta.description}
                </option>
              ))}
            </select>
            <div style={modalButtonsStyle}>
              <button
                style={buttonSecondaryStyle}
                onClick={() => {
                  setShowCreateModal(false);
                  setSelectedMetaId('');
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.backgroundColor = '#d1d5db';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor = '#e5e7eb';
                }}
              >
                取消
              </button>
              <button
                style={buttonPrimaryStyle}
                onClick={handleCreateAgent}
                disabled={loading || !selectedMetaId}
                onMouseEnter={(e) => {
                  if (!loading && selectedMetaId) {
                    e.currentTarget.style.backgroundColor = '#2563eb';
                  }
                }}
                onMouseLeave={(e) => {
                  if (!loading && selectedMetaId) {
                    e.currentTarget.style.backgroundColor = '#3b82f6';
                  }
                }}
              >
                {loading ? '创建中...' : '创建'}
              </button>
            </div>
          </div>
        </div>
      )}

      {error && <ErrorModal message={error} onClose={() => setError(null)} />}
    </Layout>
  );
}
