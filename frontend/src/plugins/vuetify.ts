import { createVuetify, ThemeDefinition } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

// 科幻主题
const cyberTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#0a0e27',
    surface: '#0f1535',
    primary: '#00ffff',
    secondary: '#ff00ff',
    accent: '#00ff88',
    error: '#ff0055',
    info: '#00ccff',
    success: '#00ff88',
    warning: '#ffaa00',
    'on-background': '#ffffff',
    'on-surface': '#ffffff',
  },
}

// 商业专业主题
const professionalTheme: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#f5f7fa',
    surface: '#ffffff',
    primary: '#1976d2',
    secondary: '#424242',
    accent: '#2196f3',
    error: '#f44336',
    info: '#2196f3',
    success: '#4caf50',
    warning: '#ff9800',
    'on-background': '#212121',
    'on-surface': '#212121',
  },
}

// 深色专业主题
const darkProfessionalTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#121212',
    surface: '#1e1e1e',
    primary: '#2196f3',
    secondary: '#90caf9',
    accent: '#03a9f4',
    error: '#f44336',
    info: '#2196f3',
    success: '#4caf50',
    warning: '#ff9800',
    'on-background': '#ffffff',
    'on-surface': '#ffffff',
  },
}

// 渐变霓虹主题
const neonTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#1a0033',
    surface: '#2d0052',
    primary: '#ff00ff',
    secondary: '#00ffff',
    accent: '#ffff00',
    error: '#ff0055',
    info: '#00ccff',
    success: '#00ff88',
    warning: '#ff6600',
    'on-background': '#ffffff',
    'on-surface': '#ffffff',
  },
}

// 极简主题
const minimalTheme: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#ffffff',
    surface: '#fafafa',
    primary: '#000000',
    secondary: '#757575',
    accent: '#424242',
    error: '#d32f2f',
    info: '#1976d2',
    success: '#388e3c',
    warning: '#f57c00',
    'on-background': '#000000',
    'on-surface': '#000000',
  },
}

// 玻璃态主题
const glassmorphismTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#1a1a2e',
    surface: 'rgba(255, 255, 255, 0.05)',
    primary: '#4facfe',
    secondary: '#00f2fe',
    accent: '#a8edea',
    error: '#ff6b6b',
    info: '#4facfe',
    success: '#6cc788',
    warning: '#feca57',
    'on-background': '#ffffff',
    'on-surface': '#ffffff',
  },
}

export default createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'cyber',
    themes: {
      cyber: cyberTheme,
      professional: professionalTheme,
      darkProfessional: darkProfessionalTheme,
      neon: neonTheme,
      minimal: minimalTheme,
      glassmorphism: glassmorphismTheme,
    },
  },
  defaults: {
    VCard: {
      elevation: 8,
    },
    VBtn: {
      elevation: 2,
    },
  },
})

