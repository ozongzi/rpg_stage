import type { CSSProperties } from 'react';

interface ErrorModalProps {
  message: string;
  onClose: () => void;
}

export function ErrorModal({ message, onClose }: ErrorModalProps) {
  const overlayStyle: CSSProperties = {
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
    boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
  };

  const headerStyle: CSSProperties = {
    fontSize: '20px',
    fontWeight: 'bold',
    color: '#dc2626',
    marginBottom: '16px',
  };

  const messageStyle: CSSProperties = {
    marginBottom: '20px',
    color: '#374151',
    lineHeight: '1.5',
  };

  const buttonStyle: CSSProperties = {
    backgroundColor: '#3b82f6',
    color: 'white',
    border: 'none',
    padding: '10px 20px',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '14px',
    fontWeight: '500',
  };

  return (
    <div style={overlayStyle} onClick={onClose}>
      <div style={modalStyle} onClick={(e) => e.stopPropagation()}>
        <h2 style={headerStyle}>错误</h2>
        <p style={messageStyle}>{message}</p>
        <button style={buttonStyle} onClick={onClose}>
          确定
        </button>
      </div>
    </div>
  );
}
