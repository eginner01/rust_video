import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ThemeName = 'cyber' | 'professional' | 'darkProfessional' | 'neon' | 'minimal' | 'glassmorphism'

export interface ThemeInfo {
  name: ThemeName
  label: string
  icon: string
  description: string
}

export const themes: ThemeInfo[] = [
  {
    name: 'cyber',
    label: '赛博科幻',
    icon: 'mdi-robot',
    description: '未来科技感的赛博朋克风格'
  },
  {
    name: 'neon',
    label: '霓虹夜光',
    icon: 'mdi-lightbulb-on',
    description: '炫彩霓虹灯效果'
  },
  {
    name: 'glassmorphism',
    label: '玻璃态',
    icon: 'mdi-glass-fragile',
    description: '现代玻璃拟态设计'
  },
  {
    name: 'professional',
    label: '商业专业',
    icon: 'mdi-briefcase',
    description: '简洁专业的商业风格'
  },
  {
    name: 'darkProfessional',
    label: '暗黑专业',
    icon: 'mdi-briefcase-outline',
    description: '深色专业商务主题'
  },
  {
    name: 'minimal',
    label: '极简主义',
    icon: 'mdi-circle-outline',
    description: '简约纯粹的极简设计'
  }
]

export const useThemeStore = defineStore('theme', () => {
  const currentTheme = ref<ThemeName>('cyber')
  const isTransitioning = ref(false)

  function setTheme(theme: ThemeName) {
    isTransitioning.value = true
    currentTheme.value = theme
    
    setTimeout(() => {
      isTransitioning.value = false
    }, 600)
  }

  function getThemeInfo(name: ThemeName): ThemeInfo | undefined {
    return themes.find(t => t.name === name)
  }

  return {
    currentTheme,
    isTransitioning,
    themes,
    setTheme,
    getThemeInfo
  }
})
