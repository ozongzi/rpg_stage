import { useState, useEffect, useRef, useCallback } from 'react';
import type { FormEvent, CSSProperties } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { apiService } from '../services/api';
import type { Agent, Conversation, Message, ApiError } from '../types';
import { Layout } from '../components/common/Layout';
import { ErrorModal } from '../components/common/ErrorModal';
import { MessageErrorWindow } from '../components/common/MessageErrorWindow';

export function ConversationPage() {
  const { agentId } = useParams<{ agentId: string }>();
  const navigate = useNavigate();
  const [agent, setAgent] = useState<Agent | null>(null);
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [messageInput, setMessageInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [messageError, setMessageError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Track latest emotion and favorability from assistant messages
  const [latestEmotion, setLatestEmotion] = useState<string>('');
  const [latestFavorability, setLatestFavorability] = useState<number | undefined>(undefined);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const loadAgent = useCallback(async () => {
    if (!agentId) return;
    try {
      const data = await apiService.getAgent(agentId);
      setAgent(data);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    }
  }, [agentId]);

  const loadConversations = useCallback(async () => {
    if (!agentId) return;
    try {
      const data = await apiService.listConversations(agentId);
      setConversations(data);
      if (data.length > 0 && !selectedConversation) {
        setSelectedConversation(data[0].id);
      }
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    }
  }, [agentId, selectedConversation]);

  const loadMessages = useCallback(async () => {
    if (!selectedConversation) return;
    try {
      const data = await apiService.listMessages(selectedConversation);
      setMessages(data);
      // Update latest emotion and favorability from most recent assistant message
      let lastAssistantMsg: Message | undefined;
      for (let i = data.length - 1; i >= 0; i--) {
        if (data[i].role === 'assistant') {
          lastAssistantMsg = data[i];
          break;
        }
      }
      if (lastAssistantMsg) {
        if (lastAssistantMsg.emotion) setLatestEmotion(lastAssistantMsg.emotion);
        if (lastAssistantMsg.favorability !== undefined) setLatestFavorability(lastAssistantMsg.favorability);
      }
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    }
  }, [selectedConversation]);

  useEffect(() => {
    if (agentId) {
      loadAgent();
      loadConversations();
    }
  }, [agentId, loadAgent, loadConversations]);

  useEffect(() => {
    if (selectedConversation) {
      loadMessages();
    }
  }, [selectedConversation, loadMessages]);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const createNewConversation = async () => {
    try {
      const { conversation_id } = await apiService.createConversation(agentId!);
      await loadConversations();
      setSelectedConversation(conversation_id);
    } catch (err) {
      const apiError = err as ApiError;
      setError(apiError.message);
    }
  };

  const handleSendMessage = async (e: FormEvent) => {
    e.preventDefault();
    if (!messageInput.trim() || !selectedConversation) return;

    const userMessage = messageInput;
    setMessageInput('');
    setLoading(true);

    // Optimistically add user message
    const newMessage: Message = {
      role: 'user',
      content: userMessage,
      timestamp: new Date().toISOString(),
    };
    setMessages((prev) => [...prev, newMessage]);

    try {
      const response = await apiService.sendMessage(selectedConversation, userMessage);
      const assistantMessage: Message = {
        role: 'assistant',
        content: response.content,
        timestamp: new Date().toISOString(),
        emotion: response.emotion,
        favorability: response.favorability,
        name: response.name,
      };
      setMessages((prev) => [...prev, assistantMessage]);
      // Update latest emotion and favorability
      if (response.emotion) setLatestEmotion(response.emotion);
      if (response.favorability !== undefined) setLatestFavorability(response.favorability);
    } catch (err) {
      const apiError = err as ApiError;
      // Show error in separate window for message sending errors
      setMessageError(apiError.message);
      // Remove optimistic user message on error
      setMessages((prev) => prev.slice(0, -1));
      setMessageInput(userMessage); // Restore input
    } finally {
      setLoading(false);
    }
  };

  const containerStyle: CSSProperties = {
    display: 'grid',
    gridTemplateColumns: '250px 1fr',
    gap: '20px',
    // height: 'calc(100vh - 120px)',
  };

  const sidebarStyle: CSSProperties = {
    position: 'sticky',
    top: '20px',
    alignSelf: 'start',
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '20px',
    boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)',
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    maxHeight: 'calc(100vh - 40px)',
    overflow: 'hidden',
  };


  const agentInfoStyle: CSSProperties = {
    borderBottom: '1px solid #e5e7eb',
    paddingBottom: '16px',
    marginBottom: '8px',
  };

  const agentNameStyle: CSSProperties = {
    fontSize: '18px',
    fontWeight: 'bold',
    color: '#1f2937',
    marginBottom: '8px',
  };

  const buttonStyle: CSSProperties = {
    padding: '10px 16px',
    backgroundColor: '#3b82f6',
    color: 'white',
    border: 'none',
    borderRadius: '6px',
    cursor: 'pointer',
    fontSize: '14px',
    fontWeight: '500',
  };

  const conversationItemStyle: CSSProperties = {
    padding: '12px',
    borderRadius: '6px',
    cursor: 'pointer',
    transition: 'background-color 0.2s',
    backgroundColor: '#f9fafb',
  };

  const selectedConversationStyle: CSSProperties = {
    ...conversationItemStyle,
    backgroundColor: '#dbeafe',
  };

  const chatContainerStyle: CSSProperties = {
    backgroundColor: 'white',
    borderRadius: '8px',
    boxShadow: '0 1px 3px 0 rgba(0, 0, 0, 0.1)',
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
  };

  const messagesContainerStyle: CSSProperties = {
    flex: 1,
    overflowY: 'auto',
    padding: '20px',
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
  };

  const messageStyle: CSSProperties = {
    padding: '12px 16px',
    borderRadius: '8px',
    maxWidth: '70%',
    wordWrap: 'break-word',
  };

  const userMessageStyle: CSSProperties = {
    ...messageStyle,
    alignSelf: 'flex-end',
    backgroundColor: '#3b82f6',
    color: 'white',
  };

  const assistantMessageStyle: CSSProperties = {
    ...messageStyle,
    alignSelf: 'flex-start',
    backgroundColor: '#f3f4f6',
    color: '#1f2937',
  };

  const inputContainerStyle: CSSProperties = {
    borderTop: '1px solid #e5e7eb',
    padding: '16px',
  };

  const formStyle: CSSProperties = {
    display: 'flex',
    gap: '12px',
  };

  const inputStyle: CSSProperties = {
    flex: 1,
    padding: '12px',
    border: '1px solid #d1d5db',
    borderRadius: '6px',
    fontSize: '14px',
  };

  const sendButtonStyle: CSSProperties = {
    padding: '12px 24px',
    backgroundColor: loading ? '#9ca3af' : '#3b82f6',
    color: 'white',
    border: 'none',
    borderRadius: '6px',
    cursor: loading ? 'not-allowed' : 'pointer',
    fontSize: '14px',
    fontWeight: '500',
  };

  const backButtonStyle: CSSProperties = {
    padding: '8px 16px',
    backgroundColor: '#6b7280',
    color: 'white',
    border: 'none',
    borderRadius: '6px',
    cursor: 'pointer',
    fontSize: '14px',
    marginBottom: '16px',
  };

  const conversationListContainerStyle: CSSProperties = {
    flex: 1,
    overflowY: 'auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
  };

  return (
    <Layout>
      <div style={containerStyle}>
        <div style={sidebarStyle}>
          <button style={backButtonStyle} onClick={() => navigate('/')}>
            ← 返回
          </button>
          {agent && (
            <div style={agentInfoStyle}>
              <div style={agentNameStyle}>{agent.name}</div>
              <div style={{ fontSize: '14px', color: '#6b7280' }}>
                情绪: {latestEmotion || agent.emotion}
              </div>
              <div style={{ fontSize: '14px', color: '#6b7280' }}>
                好感度: {latestFavorability !== undefined ? latestFavorability : agent.favorability}
              </div>
            </div>
          )}
          <button style={buttonStyle} onClick={createNewConversation}>
            + 新建对话
          </button>
          <div style={{ fontSize: '14px', fontWeight: '600', color: '#6b7280' }}>
            对话列表
          </div>
          <div style={conversationListContainerStyle}>
            {conversations.map((conv) => (
              <div
                key={conv.id}
                style={
                  selectedConversation === conv.id
                    ? selectedConversationStyle
                    : conversationItemStyle
                }
                onClick={() => setSelectedConversation(conv.id)}
              >
                {conv.title || '新对话'}
              </div>
            ))}
          </div>
        </div>

        <div style={chatContainerStyle}>
          {selectedConversation ? (
            <>
              <div style={messagesContainerStyle}>
                {messages.map((msg, idx) => (
                  <div
                    key={idx}
                    style={
                      msg.role === 'user'
                        ? userMessageStyle
                        : assistantMessageStyle
                    }
                  >
                    <div>{msg.content}</div>
                  </div>
                ))}
                <div ref={messagesEndRef} />
              </div>
              <div style={inputContainerStyle}>
                <form style={formStyle} onSubmit={handleSendMessage}>
                  <input
                    style={inputStyle}
                    type="text"
                    value={messageInput}
                    onChange={(e) => setMessageInput(e.target.value)}
                    placeholder="输入消息..."
                    disabled={loading}
                  />
                  <button style={sendButtonStyle} type="submit" disabled={loading}>
                    {loading ? '发送中...' : '发送'}
                  </button>
                </form>
              </div>
            </>
          ) : (
            <div
              style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                color: '#9ca3af',
              }}
            >
              请选择或创建一个对话
            </div>
          )}
        </div>
      </div>
      {error && <ErrorModal message={error} onClose={() => setError(null)} />}
      {messageError && (
        <MessageErrorWindow
          message={messageError}
          onClose={() => setMessageError(null)}
        />
      )}
    </Layout>
  );
}
