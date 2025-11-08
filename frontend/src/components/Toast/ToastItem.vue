<template>
  <v-alert
    :type="toast.type"
    :icon="getIcon(toast.type)"
    variant="elevated"
    closable
    class="toast-item animate__animated animate__bounceInRight"
    elevation="8"
    @click:close="$emit('close')"
  >
    <div class="d-flex align-center">
      <span class="text-body-1 font-weight-medium">{{ toast.message }}</span>
    </div>
  </v-alert>
</template>

<script setup lang="ts">
import type { Toast, ToastType } from '@/composables/useToast'

interface Props {
  toast: Toast
}

defineProps<Props>()
defineEmits<{
  close: []
}>()

function getIcon(type: ToastType): string {
  const iconMap: Record<ToastType, string> = {
    success: 'mdi-check-circle',
    error: 'mdi-alert-circle',
    warning: 'mdi-alert',
    info: 'mdi-information'
  }
  return iconMap[type]
}
</script>

<style scoped lang="scss">
.toast-item {
  pointer-events: auto;
  min-width: 320px;
  max-width: 480px;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
  transition: all 0.3s ease;

  &:hover {
    transform: translateX(-8px);
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.3);
  }
}
</style>
