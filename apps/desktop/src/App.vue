<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'

type Page = 'home' | 'models' | 'permissions' | 'account' | 'settings'
type Provider = { id: string; name: string; baseUrl: string; model: string; enabled: boolean }
type Permission = { id: string; name: string; description: string; risk: '低风险' | '中风险' | '高风险'; enabled: boolean }

const page = ref<Page>('home')
const petOnly = new URLSearchParams(window.location.search).has('pet')
const mode = ref('安静助手')
const providerDraft = ref({ name: '', baseUrl: '', model: '' })
const providers = ref<Provider[]>([])
const permissions = ref<Permission[]>([
  { id: 'system-status', name: '系统状态', description: '读取 CPU、内存和磁盘使用情况', risk: '低风险', enabled: true },
  { id: 'directory-events', name: '目录变化', description: '观察桌面、下载和用户指定目录的文件变化', risk: '低风险', enabled: true },
  { id: 'terminal-output', name: '终端输出', description: '读取命令输出以分析错误，不自动执行修复', risk: '中风险', enabled: false },
  { id: 'clipboard', name: '剪贴板', description: '读取当前剪贴板内容以辅助当前任务', risk: '中风险', enabled: false },
  { id: 'screen', name: '屏幕与 OCR', description: '分析屏幕内容，每次使用前需要确认', risk: '高风险', enabled: false },
])
const navItems: { id: Page; label: string; icon: string }[] = [
  { id: 'home', label: '今日状态', icon: '⌁' }, { id: 'models', label: '模型服务', icon: '◇' },
  { id: 'permissions', label: '权限中心', icon: '◎' }, { id: 'account', label: '账号与点数', icon: '◌' },
  { id: 'settings', label: '设置', icon: '⚙' },
]
const pageTitle = computed(() => navItems.find((item) => item.id === page.value)?.label ?? '今日状态')
const activeProvider = computed(() => providers.value.find((provider) => provider.enabled))

onMounted(() => {
  providers.value = JSON.parse(localStorage.getItem('lanmind.providers') ?? '[]')
  const savedPermissions = localStorage.getItem('lanmind.permissions')
  if (savedPermissions) permissions.value = JSON.parse(savedPermissions)
  mode.value = localStorage.getItem('lanmind.mode') ?? mode.value
})
watch(providers, (value) => localStorage.setItem('lanmind.providers', JSON.stringify(value)), { deep: true })
watch(permissions, (value) => localStorage.setItem('lanmind.permissions', JSON.stringify(value)), { deep: true })
watch(mode, (value) => localStorage.setItem('lanmind.mode', value))

function saveProvider() {
  if (!providerDraft.value.name || !providerDraft.value.baseUrl || !providerDraft.value.model) return
  providers.value.push({ id: crypto.randomUUID(), ...providerDraft.value, enabled: providers.value.length === 0 })
  providerDraft.value = { name: '', baseUrl: '', model: '' }
}
function activateProvider(id: string) {
  providers.value.forEach((provider) => (provider.enabled = provider.id === id))
}
</script>

<template>
  <div v-if="petOnly" class="pet-window" data-tauri-drag-region>
    <div class="pet-glow" data-tauri-drag-region><div class="core" data-tauri-drag-region /></div>
    <span>安静观察中</span>
  </div>
  <div v-else class="app-shell">
    <aside class="sidebar">
      <div class="brand"><div class="brand-orb"><span /></div><div><strong>澜灵</strong><small>LanMind</small></div></div>
      <nav><button v-for="item in navItems" :key="item.id" :class="{ active: page === item.id }" @click="page = item.id"><span>{{ item.icon }}</span>{{ item.label }}</button></nav>
      <div class="sidebar-foot"><span class="online-dot" /><div><strong>本地核心在线</strong><small>MVP 0.1 · 离线模式</small></div></div>
    </aside>
    <main>
      <header class="topbar"><div><small>2026 年 6 月 8 日 · 周一</small><h1>{{ pageTitle }}</h1></div><div class="top-actions"><button class="icon-button">⌕</button><button class="avatar">澜</button></div></header>
      <section v-if="page === 'home'" class="page">
        <div class="hero-card"><div><span class="eyebrow">LANMIND LOCAL CORE</span><h2>下午好，我会在该出现的时候出现。</h2><p>目前处于 {{ mode }} 模式。没有紧急事项，系统运行平稳。</p><div class="hero-actions"><button class="primary">问问澜灵</button><button @click="page = 'permissions'">查看权限</button></div></div><div class="core-visual"><div class="ring ring-a" /><div class="ring ring-b" /><div class="core" /></div></div>
        <div class="stat-grid">
          <article><small>当前模式</small><strong>{{ mode }}</strong><span class="good">低打扰运行中</span></article>
          <article><small>模型服务</small><strong>{{ activeProvider?.name ?? '尚未配置' }}</strong><span>{{ activeProvider?.model ?? '可离线使用基础能力' }}</span></article>
          <article><small>澜算云点数</small><strong>--</strong><span>登录后赠送 50 次</span></article>
          <article><small>已启用权限</small><strong>{{ permissions.filter((item) => item.enabled).length }} / {{ permissions.length }}</strong><span>敏感权限默认关闭</span></article>
        </div>
        <div class="content-grid">
          <article class="panel"><div class="panel-head"><div><small>FOCUS</small><h3>今日任务</h3></div><button>＋</button></div><div class="empty-state"><span>✓</span><strong>今天很清爽</strong><p>还没有待办任务。稍后澜灵会把值得关注的事项放在这里。</p></div></article>
          <article class="panel"><div class="panel-head"><div><small>DISCOVERIES</small><h3>主动发现</h3></div><span class="pill">0 项待处理</span></div><div class="discovery"><span class="discovery-icon">⌁</span><div><strong>观察系统已就绪</strong><p>只记录低风险系统状态，不会扫描全盘或读取敏感内容。</p></div></div><div class="discovery"><span class="discovery-icon">◇</span><div><strong>配置模型后解锁智能分析</strong><p>支持 OpenAI-compatible、本地服务与澜算云。</p></div></div></article>
        </div>
        <article class="panel quick-panel"><div class="panel-head"><div><small>QUICK ACTIONS</small><h3>快捷操作</h3></div></div><div class="quick-actions"><button><span>⌁</span>问澜灵<small>开始一次对话</small></button><button><span>▣</span>整理下载目录<small>仅生成预览</small></button><button><span>◇</span>分析项目<small>即将开放</small></button><button><span>⌕</span>搜索本地文件<small>即将开放</small></button></div></article>
      </section>
      <section v-else-if="page === 'models'" class="page">
        <div class="section-intro"><div><span class="eyebrow">CUSTOM MODEL SERVICES</span><h2>自定义模型服务</h2><p>云端和本地模型统一作为 Provider 管理。密钥安全存储将在 Tauri 接入后启用。</p></div><span class="pill">{{ providers.length }} 个 Provider</span></div>
        <div class="settings-grid"><article class="panel form-panel"><h3>添加 OpenAI-compatible Provider</h3><label>服务名称<input v-model="providerDraft.name" placeholder="例如：本地 Ollama" /></label><label>Base URL<input v-model="providerDraft.baseUrl" placeholder="http://localhost:11434/v1" /></label><label>默认模型<input v-model="providerDraft.model" placeholder="qwen3:8b" /></label><button class="primary" @click="saveProvider">保存 Provider</button></article>
          <article class="panel"><h3>已配置服务</h3><div v-if="providers.length === 0" class="empty-state compact"><span>◇</span><strong>尚未配置模型</strong><p>澜灵仍可离线运行基础能力。</p></div><button v-for="provider in providers" :key="provider.id" class="provider-row" @click="activateProvider(provider.id)"><span class="provider-mark">◇</span><span><strong>{{ provider.name }}</strong><small>{{ provider.model }} · {{ provider.baseUrl }}</small></span><span class="pill">{{ provider.enabled ? '当前使用' : '设为默认' }}</span></button></article></div>
      </section>
      <section v-else-if="page === 'permissions'" class="page"><div class="section-intro"><div><span class="eyebrow">PERMISSION FIRST</span><h2>权限透明，由你决定</h2><p>高风险能力默认关闭。当前版本不会执行命令、移动文件或读取密码与 Token。</p></div></div><article class="panel permission-list"><div v-for="permission in permissions" :key="permission.id" class="permission-row"><div><span :class="['risk', permission.risk]">{{ permission.risk }}</span><strong>{{ permission.name }}</strong><p>{{ permission.description }}</p></div><label class="switch"><input v-model="permission.enabled" type="checkbox" /><span /></label></div></article></section>
      <section v-else-if="page === 'account'" class="page"><div class="account-card panel"><div class="account-orb">澜</div><span class="eyebrow">OFFLINE READY</span><h2>不登录，也可以使用澜灵</h2><p>本地设置、自定义 Provider 与基础工具始终可离线使用。注册后将获得 50 次澜算云模型调用。</p><button class="primary">邮箱登录 / 注册（即将接入）</button></div></section>
      <section v-else class="page"><div class="section-intro"><div><span class="eyebrow">PREFERENCES</span><h2>设置</h2><p>调整澜灵的主动程度和桌面体验。</p></div></div><article class="panel form-panel settings-card"><label>运行模式<select v-model="mode"><option>安静助手</option><option>主动管家</option><option>开发者模式</option><option>服务器管家</option></select></label><label>桌宠形象<select><option>澜灵 Core</option><option>澜灵 Bot</option><option>澜灵 Fox</option></select></label><div class="setting-note">设置会立即保存在本地。SQLite 接入后将自动迁移至本地数据库。</div></article></section>
    </main>
    <aside class="insight-bar"><div class="pet-card"><div class="mini-core"><span /></div><strong>澜灵 Core</strong><small>安静观察中</small></div><div class="insight-section"><small>当前建议</small><h3>完成首次配置</h3><p>添加一个模型服务后，澜灵就能开始分析项目与主动发现。</p><button class="primary" @click="page = 'models'">配置模型服务</button></div><div class="insight-section"><small>安全状态</small><div class="safety-row"><span class="online-dot" />危险操作已禁用</div><div class="safety-row"><span class="online-dot" />敏感读取已关闭</div></div></aside>
  </div>
</template>
