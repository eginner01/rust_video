<template>
  <div class="theme-switcher">
    <v-menu 
      :close-on-content-click="false"
      location="bottom"
      transition="slide-y-transition"
    >
      <template v-slot:activator="{ props }">
        <v-btn
          v-bind="props"
          :icon="currentThemeInfo?.icon || 'mdi-palette'"
          color="primary"
          variant="flat"
          size="large"
          class="theme-btn animate__animated animate__pulse animate__infinite"
        >
        </v-btn>
      </template>

      <v-card class="theme-selector" min-width="320">
        <v-card-title class="d-flex align-center justify-space-between">
          <span class="text-h6">
            <v-icon start>mdi-palette</v-icon>
            选择主题
          </span>
          <v-chip 
            :color="currentThemeInfo?.name === 'minimal' ? 'grey' : 'primary'" 
            size="small"
            variant="flat"
          >
            {{ currentThemeInfo?.label }}
          </v-chip>
        </v-card-title>

        <v-divider></v-divider>

        <v-card-text class="pa-4">
          <v-list class="theme-list">
            <v-list-item
              v-for="theme in themes"
              :key="theme.name"
              :class="{ 'active-theme': currentTheme === theme.name }"
              @click="selectTheme(theme.name)"
              class="theme-item"
            >
              <template v-slot:prepend>
                <v-avatar 
                  :color="currentTheme === theme.name ? 'primary' : 'surface-variant'"
                  size="48"
                  class="theme-icon"
                >
                  <v-icon :icon="theme.icon" size="28"></v-icon>
                </v-avatar>
              </template>

              <v-list-item-title class="font-weight-bold">
                {{ theme.label }}
              </v-list-item-title>
              <v-list-item-subtitle class="text-caption">
                {{ theme.description }}
              </v-list-item-subtitle>

              <template v-slot:append>
                <v-icon 
                  v-if="currentTheme === theme.name"
                  icon="mdi-check-circle"
                  color="success"
                  class="animate__animated animate__bounceIn"
                ></v-icon>
              </template>
            </v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
    </v-menu>

    <!-- 主题切换动画覆盖层 -->
    <transition name="theme-overlay">
      <div v-if="themeStore.isTransitioning" class="theme-transition-overlay">
        <v-progress-circular
          indeterminate
          :color="currentThemeInfo?.name === 'minimal' ? 'grey-darken-2' : 'primary'"
          size="64"
          width="6"
        ></v-progress-circular>
        <p class="mt-4 text-h6">切换至 {{ currentThemeInfo?.label }}</p>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useThemeStore, type ThemeName } from '@/stores/theme'

const themeStore = useThemeStore()

const themes = computed(() => themeStore.themes)
const currentTheme = computed(() => themeStore.currentTheme)
const currentThemeInfo = computed(() => themeStore.getThemeInfo(currentTheme.value))

function selectTheme(themeName: ThemeName) {
  themeStore.setTheme(themeName)
}
</script>

<style scoped lang="scss">
.theme-switcher {
  position: relative;
}

.theme-btn {
  box-shadow: 0 4px 20px rgba(var(--v-theme-primary), 0.4) !important;
  transition: all 0.3s ease;

  &:hover {
    transform: scale(1.1) rotate(15deg);
    box-shadow: 0 6px 30px rgba(var(--v-theme-primary), 0.6) !important;
  }
}

.theme-selector {
  overflow: hidden;
  backdrop-filter: blur(10px);
  animation: slideIn 0.3s ease;
}

.theme-list {
  background: transparent;
}

.theme-item {
  margin: 8px 0;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid transparent;

  &:hover {
    background: rgba(var(--v-theme-primary), 0.1);
    transform: translateX(8px);
  }

  &.active-theme {
    background: rgba(var(--v-theme-primary), 0.15);
    border-color: rgba(var(--v-theme-primary), 0.5);
    
    .theme-icon {
      animation: iconPulse 1.5s ease-in-out infinite;
    }
  }
}

.theme-icon {
  transition: all 0.3s ease;
}

.theme-transition-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(var(--v-theme-background), 0.95);
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.theme-overlay-enter-active,
.theme-overlay-leave-active {
  transition: opacity 0.3s ease;
}

.theme-overlay-enter-from,
.theme-overlay-leave-to {
  opacity: 0;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.15);
  }
}
</style>

