<template>
  <v-card 
    class="parser-input-card" 
    :class="themeClass"
    elevation="12"
  >
    <v-card-text class="pa-8">
      <div class="input-section">
        <!-- 标题 -->
        <div class="section-header mb-6 animate__animated animate__fadeInDown">
          <v-icon icon="mdi-link-variant" size="32" color="primary" class="mr-3"></v-icon>
          <h2 class="text-h5 font-weight-bold">输入视频链接</h2>
        </div>

        <!-- 输入框 -->
        <v-text-field
          v-model="appStore.inputUrl"
          label="请输入视频链接"
          placeholder="支持抖音、快手、B站等多平台视频链接"
          variant="outlined"
          color="primary"
          density="comfortable"
          class="url-input animate__animated animate__fadeInUp"
          :loading="appStore.loading"
          :disabled="appStore.loading"
          clearable
          hide-details
          @keyup.enter="handleParse"
        >
          <template v-slot:prepend-inner>
            <v-icon icon="mdi-web" color="primary"></v-icon>
          </template>
        </v-text-field>

        <!-- 操作按钮 -->
        <div class="action-buttons mt-6 animate__animated animate__fadeInUp animate__delay-1s">
          <v-btn
            :loading="appStore.loading"
            :disabled="!appStore.inputUrl || appStore.loading"
            color="primary"
            size="x-large"
            variant="flat"
            prepend-icon="mdi-play-circle"
            class="parse-btn"
            @click="handleParse"
          >
            <span class="text-h6 font-weight-bold">解析视频</span>
          </v-btn>

          <v-btn
            v-if="appStore.hasResult"
            color="error"
            size="x-large"
            variant="outlined"
            prepend-icon="mdi-close-circle"
            class="clear-btn"
            @click="handleClear"
          >
            <span class="text-h6">清除结果</span>
          </v-btn>
        </div>

        <!-- 支持的平台提示 -->
        <div class="platforms-hint mt-6 animate__animated animate__fadeIn animate__delay-2s">
          <v-chip
            v-for="platform in supportedPlatforms"
            :key="platform.name"
            :prepend-icon="platform.icon"
            size="small"
            class="ma-1 platform-chip"
            variant="outlined"
          >
            {{ platform.name }}
          </v-chip>
        </div>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { useThemeStore } from '@/stores/theme'
import { useToast } from '@/composables/useToast'

const appStore = useAppStore()
const themeStore = useThemeStore()
const { showToast } = useToast()

const themeClass = computed(() => `theme-${themeStore.currentTheme}`)

const supportedPlatforms = [
  { name: '抖音', icon: 'mdi-music-note' },
  { name: '快手', icon: 'mdi-lightning-bolt' },
  { name: 'B站', icon: 'mdi-television-play' },
  { name: '小红书', icon: 'mdi-book-open-variant' },
  { name: '微博', icon: 'mdi-sina-weibo' },
  { name: '西瓜视频', icon: 'mdi-fruit-watermelon' },
  { name: '更多...', icon: 'mdi-dots-horizontal' }
]

async function handleParse() {
  if (!appStore.inputUrl.trim()) {
    showToast('请输入视频链接', 'warning')
    return
  }

  try {
    await appStore.parseUrl()
    
    if (appStore.hasError) {
      showToast(appStore.error || '解析失败', 'error')
    } else if (appStore.hasResult) {
      showToast('解析成功！', 'success')
    }
  } catch (error) {
    showToast('解析出错，请重试', 'error')
  }
}

function handleClear() {
  appStore.clearResult()
  showToast('已清除结果', 'info')
}
</script>

<style scoped lang="scss">
.parser-input-card {
  position: relative;
  overflow: hidden;
  background: rgba(var(--v-theme-surface), 0.9);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(var(--v-theme-primary), 0.2);
  transition: all 0.4s ease;

  &:hover {
    border-color: rgba(var(--v-theme-primary), 0.5);
    transform: translateY(-4px);
    box-shadow: 0 12px 40px rgba(var(--v-theme-primary), 0.3);
  }

  &.theme-cyber,
  &.theme-neon {
    &::before {
      content: '';
      position: absolute;
      top: -2px;
      left: -2px;
      right: -2px;
      bottom: -2px;
      background: linear-gradient(45deg, 
        rgba(var(--v-theme-primary), 0.5),
        rgba(var(--v-theme-secondary), 0.5),
        rgba(var(--v-theme-primary), 0.5)
      );
      background-size: 300% 300%;
      animation: gradientMove 3s ease infinite;
      z-index: -1;
      border-radius: inherit;
      filter: blur(10px);
    }
  }

  &.theme-glassmorphism {
    background: rgba(var(--v-theme-surface), 0.3);
    backdrop-filter: blur(30px);
  }
}

.section-header {
  display: flex;
  align-items: center;
  animation-delay: 0.2s;
}

.url-input {
  animation-delay: 0.4s;

  :deep(.v-field) {
    border-radius: 16px;
    background: rgba(var(--v-theme-background), 0.5);
    transition: all 0.3s ease;

    &:hover {
      background: rgba(var(--v-theme-background), 0.7);
    }
  }

  :deep(.v-field--focused) {
    box-shadow: 0 0 20px rgba(var(--v-theme-primary), 0.3);
  }
}

.action-buttons {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.parse-btn {
  flex: 1;
  min-width: 200px;
  border-radius: 16px;
  box-shadow: 0 8px 24px rgba(var(--v-theme-primary), 0.4);
  transition: all 0.3s ease;

  &:hover {
    transform: scale(1.05);
    box-shadow: 0 12px 32px rgba(var(--v-theme-primary), 0.6);
  }

  &:active {
    transform: scale(0.98);
  }
}

.clear-btn {
  flex: 1;
  min-width: 200px;
  border-radius: 16px;
  transition: all 0.3s ease;

  &:hover {
    transform: scale(1.05);
  }
}

.platforms-hint {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-items: center;
  padding-top: 16px;
  border-top: 1px dashed rgba(var(--v-theme-primary), 0.3);
}

.platform-chip {
  transition: all 0.3s ease;
  border-color: rgba(var(--v-theme-primary), 0.4);

  &:hover {
    background: rgba(var(--v-theme-primary), 0.1);
    border-color: rgba(var(--v-theme-primary), 0.8);
    transform: translateY(-2px);
  }
}

@keyframes gradientMove {
  0%, 100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}
</style>

