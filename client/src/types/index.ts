export interface User {
  id: string;
  name: string;
  email: string;
}

export interface Agent {
  id: string;
  name: string;
  emotion: string;
  favorability: number;
}

export interface AgentMeta {
  name: string;
  description: string;
  character_design: string;
  response_requirement: string;
  character_emotion_split: string;
  model: string;
}

export interface AgentMetaListItem {
  id: string;
  name: string;
  description: string;
}

export interface Conversation {
  id: string;
  title: string | null;
}

export interface Message {
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
  emotion?: string;
  favorability?: number;
  name?: string;
}

export interface ApiError {
  message: string;
  status: number;
}
