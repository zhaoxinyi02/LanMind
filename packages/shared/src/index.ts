export type ProviderType =
  | 'openai-compatible'
  | 'openai-responses'
  | 'anthropic'
  | 'gemini'
  | 'lansuan-cloud'
  | 'custom'

export interface ModelCapabilities {
  stream: boolean
  tools: boolean
  vision: boolean
  reasoning: boolean
  embeddings: boolean
}

export interface ModelProviderConfig {
  id: string
  name: string
  providerType: ProviderType
  baseUrl?: string
  apiKeyRef?: string
  defaultModel?: string
  enabled: boolean
  supports: ModelCapabilities
}

export type PermissionRisk = 'low' | 'medium' | 'high' | 'operation' | 'forbidden'
export type PermissionDecision = 'allow' | 'ask' | 'deny'

export interface PermissionConfig {
  id: string
  name: string
  risk: PermissionRisk
  decision: PermissionDecision
  scope?: string[]
}

export interface Discovery {
  id: string
  kind: 'terminal-error' | 'file-change' | 'project-change' | 'system-alert' | 'permission-request'
  title: string
  summary: string
  createdAt: string
  status: 'new' | 'reviewed' | 'dismissed'
}

export interface Task {
  id: string
  title: string
  status: 'todo' | 'doing' | 'done'
  source: 'user' | 'lanmind'
}
