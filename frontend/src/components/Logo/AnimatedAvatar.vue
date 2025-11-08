<template>
  <div class="animated-avatar-container" @mouseenter="handleMouseEnter" @mouseleave="handleMouseLeave">
    <div class="avatar-wrapper" :class="{ 'hover-active': isHovering }">
      <!-- 外层旋转光环 -->
      <div class="glow-ring ring-1"></div>
      <div class="glow-ring ring-2"></div>
      <div class="glow-ring ring-3"></div>
      
      <!-- 主头像 -->
      <div class="avatar-main">
        <v-avatar 
          :size="size" 
          class="avatar-image"
          :class="themeClass"
        >
          <v-img 
            :src="avatarSrc" 
            alt="Logo"
            cover
            class="avatar-img"
          >
            <template v-slot:placeholder>
              <v-row
                class="fill-height ma-0"
                align="center"
                justify="center"
              >
                <v-progress-circular
                  indeterminate
                  color="primary"
                ></v-progress-circular>
              </v-row>
            </template>
          </v-img>
        </v-avatar>
        
        <!-- 粒子效果 -->
        <div class="particles">
          <div 
            v-for="i in 12" 
            :key="i" 
            class="particle"
            :style="{ '--i': i }"
          ></div>
        </div>
      </div>

      <!-- 脉冲效果 -->
      <div class="pulse-effect"></div>
    </div>

    <!-- 主题标签 -->
    <div v-if="showLabel" class="theme-label animate__animated animate__fadeInUp">
      <v-chip
        :color="themeInfo?.name === 'minimal' ? 'grey-darken-2' : 'primary'"
        variant="flat"
        size="small"
        class="label-chip"
      >
        <v-icon start :icon="themeInfo?.icon || 'mdi-star'"></v-icon>
        {{ themeInfo?.label || '视频解析' }}
      </v-chip>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useThemeStore } from '@/stores/theme'

interface Props {
  size?: number
  showLabel?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 180,
  showLabel: true
})

const themeStore = useThemeStore()
const isHovering = ref(false)

const avatarSrc = computed(() => '/title.jpg')
const themeInfo = computed(() => themeStore.getThemeInfo(themeStore.currentTheme))
const themeClass = computed(() => `theme-${themeStore.currentTheme}`)

function handleMouseEnter() {
  isHovering.value = true
}

function handleMouseLeave() {
  isHovering.value = false
}
</script>

<style scoped lang="scss">
.animated-avatar-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 40px 20px;
}

.avatar-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);

  &.hover-active {
    transform: scale(1.1) rotate(5deg);

    .glow-ring {
      animation-play-state: paused;
    }

    .particles .particle {
      opacity: 1;
      transform: scale(1.5);
    }
  }
}

.avatar-main {
  position: relative;
  z-index: 2;
}

.avatar-image {
  border: 4px solid rgba(var(--v-theme-primary), 0.6);
  box-shadow: 
    0 0 30px rgba(var(--v-theme-primary), 0.4),
    0 0 60px rgba(var(--v-theme-primary), 0.2),
    inset 0 0 20px rgba(var(--v-theme-primary), 0.1);
  animation: avatarFloat 3s ease-in-out infinite;
  transition: all 0.4s ease;
  overflow: hidden;

  &:hover {
    border-color: rgba(var(--v-theme-secondary), 0.8);
    box-shadow: 
      0 0 40px rgba(var(--v-theme-secondary), 0.6),
      0 0 80px rgba(var(--v-theme-secondary), 0.3);
  }

  &.theme-cyber {
    border-color: #00ffff;
    box-shadow: 
      0 0 30px rgba(0, 255, 255, 0.5),
      0 0 60px rgba(255, 0, 255, 0.3);
  }

  &.theme-neon {
    border-color: #ff00ff;
    box-shadow: 
      0 0 30px rgba(255, 0, 255, 0.5),
      0 0 60px rgba(0, 255, 255, 0.3);
  }

  &.theme-glassmorphism {
    border-color: rgba(79, 172, 254, 0.6);
    backdrop-filter: blur(10px);
  }
}

.avatar-img {
  animation: imageRotate 20s linear infinite;
}

.glow-ring {
  position: absolute;
  border-radius: 50%;
  border: 2px solid;
  animation: ringRotate 4s linear infinite;
  opacity: 0.6;

  &.ring-1 {
    width: calc(100% + 40px);
    height: calc(100% + 40px);
    border-color: rgba(var(--v-theme-primary), 0.4);
    animation-duration: 8s;
  }

  &.ring-2 {
    width: calc(100% + 70px);
    height: calc(100% + 70px);
    border-color: rgba(var(--v-theme-secondary), 0.3);
    animation-duration: 12s;
    animation-direction: reverse;
  }

  &.ring-3 {
    width: calc(100% + 100px);
    height: calc(100% + 100px);
    border-color: rgba(var(--v-theme-accent), 0.2);
    animation-duration: 16s;
  }
}

.particles {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 100%;
  height: 100%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}

.particle {
  position: absolute;
  width: 4px;
  height: 4px;
  background: rgba(var(--v-theme-primary), 0.8);
  border-radius: 50%;
  top: 50%;
  left: 50%;
  opacity: 0;
  transition: all 0.6s ease;
  animation: particleFloat 3s ease-in-out infinite;
  animation-delay: calc(var(--i) * 0.1s);
  transform: translate(-50%, -50%) rotate(calc(var(--i) * 30deg)) translateY(0);
}

.pulse-effect {
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(var(--v-theme-primary), 0.3) 0%, transparent 70%);
  animation: pulse 2s ease-in-out infinite;
  z-index: 1;
}

.theme-label {
  animation-delay: 0.3s;
}

.label-chip {
  font-weight: 600;
  letter-spacing: 1px;
  box-shadow: 0 4px 12px rgba(var(--v-theme-primary), 0.3);
}

@keyframes avatarFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-10px) rotate(5deg);
  }
}

@keyframes imageRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes ringRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes particleFloat {
  0%, 100% {
    transform: translate(-50%, -50%) rotate(calc(var(--i) * 30deg)) translateY(0);
    opacity: 0;
  }
  50% {
    transform: translate(-50%, -50%) rotate(calc(var(--i) * 30deg)) translateY(-60px);
    opacity: 1;
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.3;
  }
  50% {
    transform: scale(1.2);
    opacity: 0;
  }
}
</style>

