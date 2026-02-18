import type { CSSProperties } from 'react';

interface MessageErrorWindowProps {
  message: string;
  onClose: () => void;
}

export function MessageErrorWindow({ message, onClose }: MessageErrorWindowProps) {
  const windowStyle: CSSProperties = {
    position: 'fixed',
    top: '50%',
    left: '50%',
    transform: 'translate(-50%, -50%)',
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '24px',
    width: '500px',
    maxWidth: '90%',
    boxShadow: '0 10px 25px rgba(0, 0, 0, 0.2)',
    border: '2px solid #ef4444',
    zIndex: 1001,
  };

  const headerStyle: CSSProperties = {
    fontSize: '22px',
    fontWeight: 'bold',
    color: '#ef4444',
    marginBottom: '16px',
    borderBottom: '2px solid #fee2e2',
    paddingBottom: '12px',
  };

  const messageStyle: CSSProperties = {
    marginBottom: '20px',
    color: '#374151',
    lineHeight: '1.6',
    fontSize: '16px',
  };

  const buttonStyle: CSSProperties = {
    backgroundColor: '#ef4444',
    color: 'white',
    border: 'none',
    padding: '12px 24px',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '14px',
    fontWeight: '600',
    width: '100%',
  };

  return (
    <div style={windowStyle}>
      <h2 style={headerStyle}>消息发送失败</h2>
      <p style={messageStyle}>{message}</p>
      <button style={buttonStyle} onClick={onClose}>
        关闭
      </button>
    </div>
  );
}
