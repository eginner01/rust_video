<template>
  <v-app>
    <!-- 主题背景效果 -->
    <div class="theme-background">
      <div class="animated-background"></div>
    </div>

    <!-- 主内容 -->
    <v-main>
      <transition name="page" mode="out-in">
        <router-view />
      </transition>
    </v-main>

    <!-- Toast通知 -->
    <ToastContainer />
  </v-app>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { useTheme } from 'vuetify'
import { useThemeStore } from './stores/theme'
import ToastContainer from './components/Toast/ToastContainer.vue'

const vuetifyTheme = useTheme()
const themeStore = useThemeStore()

// 监听主题变化并应用到 Vuetify
watch(() => themeStore.currentTheme, (newTheme) => {
  vuetifyTheme.global.name.value = newTheme
}, { immediate: true })
</script>

<style lang="scss">
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow-x: hidden;
}

.v-application {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.theme-background {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;

  .animated-background {
    position: absolute;
    width: 200%;
    height: 200%;
    top: -50%;
    left: -50%;
    background: radial-gradient(circle at 20% 50%, rgba(var(--v-theme-primary), 0.1) 0%, transparent 50%),
                radial-gradient(circle at 80% 80%, rgba(var(--v-theme-secondary), 0.1) 0%, transparent 50%);
    animation: backgroundMove 20s ease-in-out infinite;
  }
}

@keyframes backgroundMove {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
  }
  33% {
    transform: translate(-5%, -5%) rotate(5deg);
  }
  66% {
    transform: translate(5%, 5%) rotate(-5deg);
  }
}

// 页面过渡动画
.page-enter-active,
.page-leave-active {
  transition: all 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-30px) scale(0.95);
}

// 滚动条样式
::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.1);
}

::-webkit-scrollbar-thumb {
  background: rgba(var(--v-theme-primary), 0.5);
  border-radius: 5px;
  
  &:hover {
    background: rgba(var(--v-theme-primary), 0.7);
  }
}
</style>
