<template>
  <div class="home-view">
    <!-- 顶部工具栏 -->
    <div class="top-toolbar">
      <v-container fluid>
        <div class="toolbar-content">
          <div class="logo-section animate__animated animate__fadeInLeft">
            <v-icon icon="mdi-video-box" size="32" color="primary" class="mr-2"></v-icon>
            <h1 class="text-h5 font-weight-bold">视频解析工具</h1>
          </div>
          
          <div class="actions-section animate__animated animate__fadeInRight">
            <ThemeSwitcher />
          </div>
        </div>
      </v-container>
    </div>

    <!-- 主内容区 -->
    <v-container class="main-container">
      <!-- 头像展示 -->
      <div class="avatar-section">
        <AnimatedAvatar :size="avatarSize" />
      </div>

      <!-- 输入区域 -->
      <div class="input-section mb-8">
        <ParserInput />
      </div>

      <!-- 结果展示区域 -->
      <div class="result-section">
        <ParserResult />
      </div>

      <!-- 特性展示 -->
      <div v-if="!appStore.hasResult" class="features-section mt-12">
        <v-row>
          <v-col 
            v-for="(feature, index) in features" 
            :key="index"
            cols="12" 
            sm="6" 
            md="3"
          >
            <v-card
              class="feature-card"
              :class="`animate__animated animate__fadeInUp animate__delay-${index}s`"
              elevation="8"
            >
              <v-card-text class="text-center pa-6">
                <v-avatar
                  :color="feature.color"
                  size="64"
                  class="mb-4 feature-icon"
                >
                  <v-icon :icon="feature.icon" size="32"></v-icon>
                </v-avatar>
                <h3 class="text-h6 font-weight-bold mb-2">{{ feature.title }}</h3>
                <p class="text-body-2">{{ feature.description }}</p>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </div>
    </v-container>

    <!-- 底部信息 -->
    <div class="footer-section">
      <v-container>
        <div class="footer-content text-center">
          <p class="text-body-2 text-grey">
            <v-icon icon="mdi-copyright" size="16"></v-icon>
            2025 视频解析工具 | 支持多平台视频解析
          </p>
          <div class="mt-2">
            <v-chip
              size="x-small"
              variant="outlined"
              class="ma-1"
            >
              <v-icon icon="mdi-speedometer" size="16" start></v-icon>
              快速解析
            </v-chip>
            <v-chip
              size="x-small"
              variant="outlined"
              class="ma-1"
            >
              <v-icon icon="mdi-shield-check" size="16" start></v-icon>
              安全可靠
            </v-chip>
            <v-chip
              size="x-small"
              variant="outlined"
              class="ma-1"
            >
              <v-icon icon="mdi-web" size="16" start></v-icon>
              多平台支持
            </v-chip>
          </div>
        </div>
      </v-container>
    </div>

    <!-- 装饰性粒子背景 -->
    <div class="decorative-particles">
      <div 
        v-for="i in 20" 
        :key="i" 
        class="particle"
        :style="{
          '--delay': `${i * 0.2}s`,
          '--duration': `${3 + i * 0.3}s`,
          '--x': `${Math.random() * 100}vw`,
          '--y': `${Math.random() * 100}vh`
        }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useDisplay } from 'vuetify'
import { useAppStore } from '@/stores/app'
import AnimatedAvatar from '@/components/Logo/AnimatedAvatar.vue'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import ParserInput from '@/components/Parser/ParserInput.vue'
import ParserResult from '@/components/Parser/ParserResult.vue'

const appStore = useAppStore()
const { mdAndUp } = useDisplay()

const avatarSize = computed(() => mdAndUp.value ? 180 : 120)

const features = [
  {
    icon: 'mdi-lightning-bolt',
    title: '极速解析',
    description: '毫秒级响应，快速获取视频信息',
    color: 'primary'
  },
  {
    icon: 'mdi-shield-check',
    title: '安全稳定',
    description: '无需登录，保护您的隐私安全',
    color: 'success'
  },
  {
    icon: 'mdi-devices',
    title: '多平台',
    description: '支持抖音、快手、B站等主流平台',
    color: 'info'
  },
  {
    icon: 'mdi-refresh',
    title: '实时更新',
    description: '持续更新，适配最新平台规则',
    color: 'warning'
  }
]
</script>

<style scoped lang="scss">
.home-view {
  min-height: 100vh;
  position: relative;
  padding-bottom: 80px;
}

.top-toolbar {
  position: sticky;
  top: 0;
  z-index: 100;
  background: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(var(--v-theme-primary), 0.1);
  padding: 16px 0;
}

.toolbar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo-section {
  display: flex;
  align-items: center;
}

.actions-section {
  display: flex;
  gap: 12px;
  align-items: center;
}

.main-container {
  position: relative;
  z-index: 1;
  padding-top: 40px;
}

.avatar-section {
  display: flex;
  justify-content: center;
  margin-bottom: 40px;
}

.input-section {
  max-width: 900px;
  margin: 0 auto;
}

.result-section {
  max-width: 1200px;
  margin: 0 auto;
}

.features-section {
  .feature-card {
    height: 100%;
    background: rgba(var(--v-theme-surface), 0.9);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(var(--v-theme-primary), 0.1);
    transition: all 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);

    &:hover {
      transform: translateY(-12px) scale(1.05);
      border-color: rgba(var(--v-theme-primary), 0.5);
      box-shadow: 0 16px 48px rgba(var(--v-theme-primary), 0.3);

      .feature-icon {
        transform: rotate(360deg) scale(1.2);
      }
    }
  }

  .feature-icon {
    transition: all 0.6s ease;
  }
}

.footer-section {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 24px 0;
  background: rgba(var(--v-theme-surface), 0.6);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(var(--v-theme-primary), 0.1);
}

.decorative-particles {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
  overflow: hidden;

  .particle {
    position: absolute;
    width: 4px;
    height: 4px;
    background: rgba(var(--v-theme-primary), 0.3);
    border-radius: 50%;
    animation: particleFloat var(--duration) ease-in-out infinite;
    animation-delay: var(--delay);
    left: var(--x);
    top: var(--y);
  }
}

@keyframes particleFloat {
  0%, 100% {
    transform: translate(0, 0) scale(1);
    opacity: 0;
  }
  10% {
    opacity: 1;
  }
  90% {
    opacity: 1;
  }
  25% {
    transform: translate(-30px, -40px) scale(1.2);
  }
  50% {
    transform: translate(40px, -30px) scale(1.5);
  }
  75% {
    transform: translate(-20px, 30px) scale(1.3);
  }
}

// 响应式调整
@media (max-width: 960px) {
  .top-toolbar {
    .toolbar-content {
      flex-direction: column;
      gap: 12px;
    }
  }

  .features-section {
    margin-top: 40px !important;
  }
}

@media (max-width: 600px) {
  .logo-section {
    h1 {
      font-size: 1.2rem !important;
    }
  }

  .main-container {
    padding-top: 20px;
  }

  .avatar-section {
    margin-bottom: 20px;
  }
}
</style>
