<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  Bot, Check, ChevronRight, CircleUserRound, Cloud, Download, Eye, FolderSearch,
  Gauge, Home, KeyRound, Laptop, Mail, MessageCircle, RefreshCw, Settings,
  ShieldCheck, Sparkles, X, Zap,
} from 'lucide-vue-next'

type Page = 'home' | 'models' | 'permissions' | 'account' | 'settings'
type Provider = { id: string; name: string; baseUrl: string; model: string; enabled: boolean }
type Permission = { id: string; name: string; description: string; risk: '低风险' | '中风险' | '高风险'; enabled: boolean }
type UpdateInfo = { currentVersion: string; latestVersion: string; releaseUrl: string; notes: string; downloadUrl: string; available: boolean }

const currentVersion = '0.1.2'
const isTauri = '__TAURI_INTERNALS__' in window
const page = ref<Page>('home')
const petOnly = new URLSearchParams(window.location.search).has('pet')
const mode = ref('安静助手')
const providerDraft = ref({ name: '', baseUrl: '', model: '' })
const providers = ref<Provider[]>([])
const updateState = ref<'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'latest' | 'error'>('idle')
const updateInfo = ref<UpdateInfo>()
const installerPath = ref('')
const updateMessage = ref('')
const permissions = ref<Permission[]>([
  { id: 'system-status', name: '系统状态', description: '读取 CPU、内存和磁盘使用情况', risk: '低风险', enabled: true },
  { id: 'directory-events', name: '目录变化', description: '观察下载和用户指定目录的文件变化', risk: '低风险', enabled: true },
  { id: 'terminal-output', name: '终端输出', description: '读取命令输出以分析错误，不自动执行修复', risk: '中风险', enabled: false },
  { id: 'clipboard', name: '剪贴板', description: '读取当前剪贴板内容以辅助当前任务', risk: '中风险', enabled: false },
  { id: 'screen', name: '屏幕与 OCR', description: '分析屏幕内容，每次使用前需要确认', risk: '高风险', enabled: false },
])
const navItems = [
  { id: 'home' as Page, label: '首页', icon: Home },
  { id: 'models' as Page, label: '模型服务', icon: Bot },
  { id: 'permissions' as Page, label: '权限中心', icon: ShieldCheck },
  { id: 'account' as Page, label: '账号', icon: CircleUserRound },
  { id: 'settings' as Page, label: '设置', icon: Settings },
]
const activeProvider = computed(() => providers.value.find((provider) => provider.enabled))
const enabledPermissions = computed(() => permissions.value.filter((item) => item.enabled).length)

onMounted(() => {
  document.documentElement.classList.toggle('pet-mode', petOnly)
  providers.value = JSON.parse(localStorage.getItem('lanmind.providers') ?? '[]')
  const savedPermissions = localStorage.getItem('lanmind.permissions')
  if (savedPermissions) permissions.value = JSON.parse(savedPermissions)
  mode.value = localStorage.getItem('lanmind.mode') ?? mode.value
  if (!petOnly && isTauri) void checkUpdate()
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
async function hidePet() {
  await invoke('hide_pet_window').catch(() => undefined)
}
async function checkUpdate() {
  updateState.value = 'checking'
  updateMessage.value = ''
  try {
    updateInfo.value = await invoke<UpdateInfo>('check_for_update')
    updateState.value = updateInfo.value.available ? 'available' : 'latest'
  } catch (error) {
    updateState.value = 'error'
    updateMessage.value = String(error)
  }
}
async function downloadUpdate() {
  if (!updateInfo.value) return
  updateState.value = 'downloading'
  try {
    installerPath.value = await invoke<string>('download_update', { downloadUrl: updateInfo.value.downloadUrl })
    updateState.value = 'ready'
  } catch (error) {
    updateState.value = 'error'
    updateMessage.value = String(error)
  }
}
async function installUpdate() {
  await invoke('install_update', { installerPath: installerPath.value })
}
</script>

<template>
  <div v-if="petOnly" class="pet-window" data-tauri-drag-region>
    <button class="pet-close" title="隐藏桌宠" @click="hidePet"><X :size="14" /></button>
    <div class="pet-mark" data-tauri-drag-region><Sparkles :size="32" stroke-width="1.7" /></div>
    <span>澜灵在线</span>
  </div>

  <div v-else class="app-shell">
    <aside class="sidebar">
      <div class="brand"><div class="logo"><Sparkles :size="20" /></div><div><strong>澜灵</strong><small>LanMind</small></div></div>
      <nav>
        <button v-for="item in navItems" :key="item.id" :class="{ active: page === item.id }" @click="page = item.id">
          <component :is="item.icon" :size="17" stroke-width="1.8" />{{ item.label }}
        </button>
      </nav>
      <div class="sidebar-foot"><span class="online-dot" /><div><strong>本地核心在线</strong><small>v{{ currentVersion }}</small></div></div>
    </aside>

    <main>
      <header class="topbar">
        <div><small>LANMIND WORKSPACE</small><h1>{{ navItems.find((item) => item.id === page)?.label }}</h1></div>
        <button class="profile-button"><span>澜</span><div><strong>离线用户</strong><small>本地模式</small></div><ChevronRight :size="15" /></button>
      </header>

      <section v-if="page === 'home'" class="page">
        <div v-if="updateState === 'available'" class="update-banner"><RefreshCw :size="17" /><div><strong>发现新版本 v{{ updateInfo?.latestVersion }}</strong><p>前往设置下载并安装最新版本。</p></div><button @click="page = 'settings'">查看更新<ChevronRight :size="15" /></button></div>
        <div class="welcome-card">
          <div><span class="status-label"><span class="online-dot" />{{ mode }}运行中</span><h2>下午好，电脑状态一切正常。</h2><p>澜灵会保持安静，只在值得你注意的时候出现。</p><div class="button-row"><button class="primary"><MessageCircle :size="16" />问问澜灵</button><button @click="page = 'permissions'"><ShieldCheck :size="16" />查看权限</button></div></div>
          <div class="welcome-symbol"><Sparkles :size="40" stroke-width="1.4" /></div>
        </div>

        <div class="stat-grid">
          <article><div class="stat-icon blue"><Bot :size="18" /></div><small>模型服务</small><strong>{{ activeProvider?.name ?? '尚未配置' }}</strong><span>{{ activeProvider?.model ?? '添加后启用智能分析' }}</span></article>
          <article><div class="stat-icon green"><ShieldCheck :size="18" /></div><small>已启用权限</small><strong>{{ enabledPermissions }} / {{ permissions.length }}</strong><span>敏感能力默认关闭</span></article>
          <article><div class="stat-icon violet"><Cloud :size="18" /></div><small>澜算云点数</small><strong>--</strong><span>登录后赠送 50 次</span></article>
        </div>

        <div class="content-grid">
          <article class="panel"><div class="panel-head"><div><small>TODAY</small><h3>今日任务</h3></div><button class="quiet-button">添加</button></div><div class="empty-state"><div class="empty-icon"><Check :size="20" /></div><strong>今天很清爽</strong><p>暂时没有待办事项。</p></div></article>
          <article class="panel"><div class="panel-head"><div><small>DISCOVERIES</small><h3>主动发现</h3></div><span class="count">2</span></div>
            <div class="list-row"><div class="row-icon"><Eye :size="17" /></div><div><strong>观察系统已就绪</strong><p>只记录你允许的低风险状态。</p></div><ChevronRight :size="16" /></div>
            <div class="list-row"><div class="row-icon"><Bot :size="17" /></div><div><strong>配置模型服务</strong><p>启用项目分析和智能建议。</p></div><ChevronRight :size="16" /></div>
          </article>
        </div>

        <article class="panel quick-panel"><div class="panel-head"><div><small>QUICK ACTIONS</small><h3>快捷操作</h3></div></div><div class="quick-actions">
          <button><MessageCircle :size="18" /><strong>问澜灵</strong><small>开始一次对话</small></button>
          <button><Download :size="18" /><strong>整理下载目录</strong><small>仅生成预览</small></button>
          <button><Gauge :size="18" /><strong>分析项目</strong><small>即将开放</small></button>
          <button><FolderSearch :size="18" /><strong>搜索本地文件</strong><small>即将开放</small></button>
        </div></article>
      </section>

      <section v-else-if="page === 'models'" class="page">
        <div class="section-intro"><div><small>MODEL PROVIDERS</small><h2>模型服务</h2><p>本地模型与云端模型使用相同的 Provider 配置方式。</p></div><span class="soft-badge">{{ providers.length }} 个服务</span></div>
        <div class="settings-grid"><article class="panel form-panel"><h3>添加 OpenAI-compatible 服务</h3><label>服务名称<input v-model="providerDraft.name" placeholder="例如：本地 Ollama" /></label><label>Base URL<input v-model="providerDraft.baseUrl" placeholder="http://localhost:11434/v1" /></label><label>默认模型<input v-model="providerDraft.model" placeholder="qwen3:8b" /></label><button class="primary" @click="saveProvider"><Bot :size="16" />保存服务</button></article>
          <article class="panel"><h3>已配置服务</h3><div v-if="providers.length === 0" class="empty-state tall"><div class="empty-icon"><Bot :size="20" /></div><strong>尚未配置模型</strong><p>澜灵仍可离线运行基础功能。</p></div><button v-for="provider in providers" :key="provider.id" class="provider-row" @click="activateProvider(provider.id)"><div class="row-icon"><Bot :size="17" /></div><span><strong>{{ provider.name }}</strong><small>{{ provider.model }} · {{ provider.baseUrl }}</small></span><span class="soft-badge">{{ provider.enabled ? '当前使用' : '设为默认' }}</span></button></article></div>
      </section>

      <section v-else-if="page === 'permissions'" class="page">
        <div class="section-intro"><div><small>PRIVACY FIRST</small><h2>权限中心</h2><p>每项观察与操作能力都由你决定。危险操作始终需要确认。</p></div></div>
        <article class="panel permission-list"><div v-for="permission in permissions" :key="permission.id" class="permission-row"><div class="row-icon"><KeyRound :size="17" /></div><div class="permission-copy"><div><strong>{{ permission.name }}</strong><span :class="['risk', permission.risk]">{{ permission.risk }}</span></div><p>{{ permission.description }}</p></div><label class="switch"><input v-model="permission.enabled" type="checkbox" /><span /></label></div></article>
      </section>

      <section v-else-if="page === 'account'" class="page">
        <div class="section-intro"><div><small>ACCOUNT</small><h2>账号与点数</h2><p>不登录也能使用本地能力。登录后可使用澜算云服务。</p></div></div>
        <div class="account-grid"><article class="panel account-card"><div class="account-icon"><CircleUserRound :size="30" /></div><h3>当前为离线用户</h3><p>本地设置、自定义 Provider 与基础工具始终可用。</p><button class="primary"><Mail :size="16" />邮箱登录 / 注册</button></article>
          <article class="panel mail-card"><div class="panel-head"><div><small>SYSTEM EMAIL</small><h3>系统邮件</h3></div><Mail :size="18" /></div><div class="info-row"><span>发件地址</span><strong>lanmind@lansuan.cc</strong></div><div class="info-row"><span>状态</span><strong class="success-text">等待云端账号服务接入</strong></div><p class="secure-note">SMTP 凭据仅保存在云端环境变量中，不会打包进桌面应用或提交到 GitHub。</p></article></div>
      </section>

      <section v-else class="page">
        <div class="section-intro"><div><small>PREFERENCES</small><h2>设置</h2><p>调整运行方式、桌宠和软件更新。</p></div></div>
        <div class="settings-stack">
          <article class="panel setting-section"><div><div class="row-icon"><Laptop :size="17" /></div><div><h3>运行模式</h3><p>控制澜灵的主动提醒程度。</p></div></div><select v-model="mode"><option>安静助手</option><option>主动管家</option><option>开发者模式</option><option>服务器管家</option></select></article>
          <article class="panel setting-section"><div><div class="row-icon"><Sparkles :size="17" /></div><div><h3>桌宠形象</h3><p>当前使用简约版澜灵 Core。</p></div></div><select><option>澜灵 Core</option><option disabled>澜灵 Bot（即将开放）</option><option disabled>澜灵 Fox（即将开放）</option></select></article>
          <article class="panel update-card"><div class="update-copy"><div class="row-icon"><RefreshCw :size="17" /></div><div><h3>软件更新</h3><p>当前版本 v{{ currentVersion }} · 更新来自 GitHub Releases</p><span v-if="updateState === 'available'" class="update-note">发现 v{{ updateInfo?.latestVersion }}，可以下载安装。</span><span v-else-if="updateState === 'ready'" class="update-note">安装包已下载，点击后将退出澜灵并自动安装。</span><span v-else-if="updateState === 'latest'" class="success-text">已经是最新版本。</span><span v-else-if="updateState === 'error'" class="error-text">{{ updateMessage }}</span></div></div>
            <button v-if="updateState === 'available'" class="primary" @click="downloadUpdate"><Download :size="16" />下载更新</button>
            <button v-else-if="updateState === 'ready'" class="primary" @click="installUpdate"><Zap :size="16" />重启并安装</button>
            <button v-else class="secondary" :disabled="updateState === 'checking' || updateState === 'downloading'" @click="checkUpdate"><RefreshCw :size="16" :class="{ spinning: updateState === 'checking' || updateState === 'downloading' }" />{{ updateState === 'checking' ? '正在检查' : updateState === 'downloading' ? '正在下载' : '检查更新' }}</button>
          </article>
        </div>
      </section>
    </main>
  </div>
</template>
