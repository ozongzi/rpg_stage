import axios from 'axios';
import type { AxiosError, AxiosInstance } from 'axios';
import { API_BASE_URL, SESSION_TOKEN_KEY } from '../utils/constants';
import type { User, Agent, AgentMeta, AgentMetaListItem, Conversation, Message } from '../types';

class ApiService {
  private api: AxiosInstance;

  constructor() {
    this.api = axios.create({
      baseURL: API_BASE_URL,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add interceptor to include auth token
    this.api.interceptors.request.use((config) => {
      const token = localStorage.getItem(SESSION_TOKEN_KEY);
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    });
  }

  // Error handler
  private handleError(error: unknown): never {
    if (axios.isAxiosError(error)) {
      const axiosError = error as AxiosError<string>;
      const message = axiosError.response?.data || axiosError.message || 'Unknown error occurred';
      const status = axiosError.response?.status || 500;
      throw { message, status };
    }
    throw { message: 'Unknown error occurred', status: 500 };
  }

  // Health check
  async healthCheck(): Promise<void> {
    try {
      await this.api.get('/health');
    } catch (error) {
      this.handleError(error);
    }
  }

  // Auth
  async login(email: string, password: string): Promise<string> {
    try {
      const params = new URLSearchParams();
      params.append('email', email);
      params.append('password', password);
      
      const response = await this.api.post<string>('/auth/session', params, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
      });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async logout(): Promise<void> {
    try {
      await this.api.delete('/auth/session');
    } catch (error) {
      this.handleError(error);
    }
  }

  // Users
  async createUser(name: string, email: string, password: string): Promise<{ user_id: string }> {
    try {
      const params = new URLSearchParams();
      params.append('name', name);
      params.append('email', email);
      params.append('password', password);
      
      const response = await this.api.post<{ user_id: string }>('/users', params, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
      });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async listUsers(): Promise<User[]> {
    try {
      const response = await this.api.get<User[]>('/users');
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  // Agent Metas
  async createAgentMeta(meta: AgentMeta): Promise<{ agent_meta_id: string }> {
    try {
      const params = new URLSearchParams();
      params.append('name', meta.name);
      params.append('description', meta.description);
      params.append('character_design', meta.character_design);
      params.append('response_requirement', meta.response_requirement);
      params.append('character_emotion_split', meta.character_emotion_split);
      params.append('model', meta.model);
      
      const response = await this.api.post<{ agent_meta_id: string }>('/agent_metas', params, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
      });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async listAgentMetas(): Promise<AgentMetaListItem[]> {
    try {
      const response = await this.api.get<AgentMetaListItem[]>('/agent_metas');
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  // Agents
  async createAgent(agentMetadataId: string): Promise<{ agent_id: string }> {
    try {
      const params = new URLSearchParams();
      params.append('agent_metadata_id', agentMetadataId);
      
      const response = await this.api.post<{ agent_id: string }>('/agents', params, {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
      });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async listAgents(): Promise<Agent[]> {
    try {
      const response = await this.api.get<Agent[]>('/agents');
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async getAgent(id: string): Promise<Agent> {
    try {
      const response = await this.api.get<Agent>(`/agents/${id}`);
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  // Conversations
  async createConversation(agentId: string): Promise<{ conversation_id: string }> {
    try {
      const response = await this.api.post<{ conversation_id: string }>(`/agents/${agentId}/conversations`);
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async listConversations(agentId: string): Promise<Conversation[]> {
    try {
      const response = await this.api.get<Conversation[]>(`/agents/${agentId}/conversations`);
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  // Messages
  async sendMessage(conversationId: string, content: string): Promise<{ content: string; emotion?: string; favorability?: number; name?: string }> {
    try {
      const response = await this.api.post<{ content: string; emotion?: string; favorability?: number; name?: string }>(`/conversations/${conversationId}/messages`, {
        content,
      });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async listMessages(conversationId: string): Promise<Message[]> {
    try {
      const response = await this.api.get<Message[]>(`/conversations/${conversationId}/messages`);
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }
}

export const apiService = new ApiService();
