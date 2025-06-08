<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import Button from 'primevue/button'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
}>()

const router = useRouter()
const route = useRoute()

const menuItems = ref([
  {
    label: 'Hosts',
    icon: 'pi pi-server',
    route: '/',
    color: 'var(--green-500)',
  },
  {
    label: 'SSL Certificates',
    icon: 'pi pi-shield',
    route: '/ssl-certificates',
    color: 'var(--blue-500)',
  },
])

const navigateTo = (routePath: string) => {
  router.push(routePath)
  if (window.innerWidth <= 768) {
    emit('update:visible', false)
  }
}

const isActiveRoute = (routePath: string) => {
  return route.path === routePath
}

const closeSidebar = () => {
  emit('update:visible', false)
}
</script>

<template>
  <div class="sidebar-overlay" v-if="visible" @click="closeSidebar"></div>

  <aside class="sidebar" :class="{ 'sidebar-visible': visible }">
    <div class="sidebar-header">
      <div class="sidebar-title">
        <i class="pi pi-server sidebar-icon"></i>
        <span>Caddy Manager</span>
      </div>
      <Button
        icon="pi pi-times"
        class="close-btn"
        @click="closeSidebar"
        severity="secondary"
        text
        size="small"
      />
    </div>

    <nav class="sidebar-nav">
      <ul class="nav-list">
        <li v-for="item in menuItems" :key="item.route" class="nav-item">
          <button
            class="nav-link"
            :class="{ 'nav-link-active': isActiveRoute(item.route) }"
            @click="navigateTo(item.route)"
          >
            <div class="nav-icon-wrapper">
              <i :class="item.icon" class="nav-icon" :style="{ color: item.color }"></i>
            </div>
            <span class="nav-label">{{ item.label }}</span>
            <div class="nav-indicator" v-if="isActiveRoute(item.route)"></div>
          </button>
        </li>
      </ul>
    </nav>

    <div class="sidebar-footer">
      <div class="app-info">
        <div class="app-version">
          <i class="pi pi-info-circle"></i>
          <span>Version 1.0.0</span>
        </div>
        <div class="caddy-status">
          <div class="status-dot"></div>
          <span>Caddy Active</span>
        </div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  z-index: 999;
  display: none;
}

.sidebar {
  position: fixed;
  top: 70px;
  left: -280px;
  width: 280px;
  height: calc(100vh - 70px);
  background: var(--surface-card);
  border-right: 1px solid var(--surface-border);
  display: flex;
  flex-direction: column;
  transition: left 0.3s ease;
  z-index: 1000;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
}

.sidebar-visible {
  left: 0;
}

.sidebar-header {
  padding: 1.5rem 1.5rem 1rem;
  border-bottom: 1px solid var(--surface-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-weight: 600;
  color: var(--text-color);
}

.sidebar-icon {
  font-size: 1.25rem;
  color: var(--primary-color);
}

.close-btn {
  width: 2rem;
  height: 2rem;
  display: none;
}

.sidebar-nav {
  flex: 1;
  padding: 1rem 0;
  overflow-y: auto;
}

.nav-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.nav-item {
  margin: 0.25rem 0;
}

.nav-link {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  border: none;
  background: none;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  text-align: left;
  color: var(--text-color);
  font-size: 0.9rem;
}

.nav-link:hover {
  background: var(--surface-hover);
  transform: translateX(4px);
}

.nav-link-active {
  background: var(--primary-50);
  color: var(--primary-color);
  font-weight: 600;
}

.nav-link-active .nav-icon {
  color: var(--primary-color) !important;
}

.nav-icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.5rem;
  background: var(--surface-100);
  transition: all 0.2s ease;
}

.nav-link:hover .nav-icon-wrapper {
  background: var(--surface-200);
  transform: scale(1.05);
}

.nav-link-active .nav-icon-wrapper {
  background: var(--primary-100);
}

.nav-icon {
  font-size: 1.1rem;
  transition: color 0.2s ease;
}

.nav-label {
  flex: 1;
  font-weight: 500;
}

.nav-indicator {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 60%;
  background: var(--primary-color);
  border-radius: 2px 0 0 2px;
}

.sidebar-footer {
  padding: 1.5rem;
  border-top: 1px solid var(--surface-border);
  background: var(--surface-50);
}

.app-info {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.app-version,
.caddy-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.8rem;
  color: var(--text-color-secondary);
}

.caddy-status .status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--green-500);
  animation: pulse 2s infinite;
}

@media (max-width: 768px) {
  .sidebar-overlay {
    display: block;
  }

  .sidebar {
    top: 70px;
    left: -100%;
    width: 280px;
  }

  .close-btn {
    display: flex;
  }

  .sidebar-visible {
    left: 0;
  }
}

@media (min-width: 769px) {
  .sidebar-overlay {
    display: none !important;
  }
}
</style>
