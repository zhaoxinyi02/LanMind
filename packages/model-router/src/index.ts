import type { ModelProviderConfig } from '@lanmind/shared'

export function getActiveProvider(providers: ModelProviderConfig[]): ModelProviderConfig | undefined {
  return providers.find((provider) => provider.enabled)
}

export function validateProvider(provider: ModelProviderConfig): string[] {
  const errors: string[] = []
  if (!provider.name.trim()) errors.push('Provider 名称不能为空')
  if (provider.providerType !== 'lansuan-cloud' && !provider.baseUrl?.trim()) errors.push('Base URL 不能为空')
  if (!provider.defaultModel?.trim()) errors.push('默认模型不能为空')
  return errors
}
