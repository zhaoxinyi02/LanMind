import type { PermissionConfig, PermissionDecision, PermissionRisk } from '@lanmind/shared'

export const DEFAULT_PERMISSION_DECISIONS: Record<PermissionRisk, PermissionDecision> = {
  low: 'allow',
  medium: 'ask',
  high: 'ask',
  operation: 'ask',
  forbidden: 'deny',
}

export function resolvePermission(permission: PermissionConfig): PermissionDecision {
  if (permission.risk === 'forbidden') return 'deny'
  return permission.decision ?? DEFAULT_PERMISSION_DECISIONS[permission.risk]
}
