<template>
  <teleport to="body">
    <div class="toast-container">
      <transition-group name="toast" tag="div">
        <ToastItem
          v-for="toast in toasts"
          :key="toast.id"
          :toast="toast"
          @close="removeToast(toast.id)"
        />
      </transition-group>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { useToast } from '@/composables/useToast'
import ToastItem from './ToastItem.vue'

const { toasts, removeToast } = useToast()
</script>

<style scoped lang="scss">
.toast-container {
  position: fixed;
  top: 24px;
  right: 24px;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  gap: 12px;
  pointer-events: none;
}

.toast-enter-active {
  animation: toastSlideIn 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

.toast-leave-active {
  animation: toastSlideOut 0.3s ease;
}

.toast-move {
  transition: transform 0.3s ease;
}

@keyframes toastSlideIn {
  from {
    opacity: 0;
    transform: translateX(100%) scale(0.8);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes toastSlideOut {
  from {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateX(100%) scale(0.8);
  }
}
</style>
