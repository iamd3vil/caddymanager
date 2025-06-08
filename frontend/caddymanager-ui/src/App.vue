<script setup lang="ts">
import { RouterView } from 'vue-router'
import Toast from 'primevue/toast'
import ConfirmDialog from 'primevue/confirmdialog'
import AppHeader from './components/AppHeader.vue'
import AppSidebar from './components/AppSidebar.vue'
import { ref } from 'vue'

const sidebarVisible = ref(false)

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value
}
</script>

<template>
  <div class="app-container">
    <Toast />
    <ConfirmDialog />

    <AppHeader @toggle-sidebar="toggleSidebar" />

    <div class="app-content">
      <AppSidebar :visible="sidebarVisible" @update:visible="sidebarVisible = $event" />

      <main class="main-content" :class="{ 'with-sidebar': sidebarVisible }">
        <div class="content-wrapper">
          <RouterView />
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  min-height: 100vh;
  background: var(--surface-ground);
  display: flex;
  flex-direction: column;
}

.app-content {
  flex: 1;
  display: flex;
  position: relative;
}

.main-content {
  flex: 1;
  padding: 2rem;
  transition: margin-left 0.3s ease;
  background: var(--surface-ground);
}

.main-content.with-sidebar {
  margin-left: 280px;
}

.content-wrapper {
  max-width: 1400px;
  margin: 0 auto;
}

@media (max-width: 768px) {
  .main-content {
    padding: 1rem;
  }

  .main-content.with-sidebar {
    margin-left: 0;
  }
}
</style>
