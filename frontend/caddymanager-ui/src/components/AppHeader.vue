<script setup lang="ts">
import Button from 'primevue/button'
import { ref } from 'vue'

defineEmits<{
  'toggle-sidebar': []
}>()

const currentTime = ref(new Date().toLocaleTimeString())

// Update time every second
setInterval(() => {
  currentTime.value = new Date().toLocaleTimeString()
}, 1000)
</script>

<template>
  <header class="app-header">
    <div class="header-content">
      <div class="header-left">
        <Button
          icon="pi pi-bars"
          class="sidebar-toggle"
          @click="$emit('toggle-sidebar')"
          severity="secondary"
          text
        />
        <div class="app-title">
          <i class="pi pi-server header-icon"></i>
          <h1>Caddy Manager</h1>
        </div>
      </div>

      <div class="header-right">
        <div class="status-indicator">
          <div class="status-dot active"></div>
          <span>System Online</span>
        </div>
        <div class="current-time">
          {{ currentTime }}
        </div>
      </div>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  background: var(--surface-card);
  border-bottom: 1px solid var(--surface-border);
  height: 70px;
  display: flex;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 1000;
}

.header-content {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 2rem;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.sidebar-toggle {
  width: 2.5rem;
  height: 2.5rem;
}

.app-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.header-icon {
  font-size: 1.5rem;
  color: var(--primary-color);
}

.app-title h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-color);
  background: linear-gradient(135deg, var(--primary-color) 0%, var(--primary-color-text) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 2rem;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--green-50);
  border: 1px solid var(--green-200);
  border-radius: 2rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--green-700);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--green-500);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    transform: scale(0.95);
    box-shadow: 0 0 0 0 rgba(34, 197, 94, 0.7);
  }

  70% {
    transform: scale(1);
    box-shadow: 0 0 0 10px rgba(34, 197, 94, 0);
  }

  100% {
    transform: scale(0.95);
    box-shadow: 0 0 0 0 rgba(34, 197, 94, 0);
  }
}

.current-time {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: var(--text-color-secondary);
  background: var(--surface-100);
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  border: 1px solid var(--surface-border);
}

@media (max-width: 768px) {
  .header-content {
    padding: 0 1rem;
  }

  .app-title h1 {
    font-size: 1.25rem;
  }

  .header-right {
    gap: 1rem;
  }

  .status-indicator {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
  }

  .current-time {
    font-size: 0.75rem;
    padding: 0.375rem 0.75rem;
  }
}
</style>
