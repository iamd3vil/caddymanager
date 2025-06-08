<script setup lang="ts">
import { ref } from 'vue'
import Button from 'primevue/button'
import Card from 'primevue/card'
import { useToast } from 'primevue/usetoast'

const toast = useToast()

const showComingSoon = () => {
  toast.add({
    severity: 'info',
    summary: 'Coming Soon',
    detail: 'SSL Certificate management will be available in the next update',
    life: 4000,
  })
}
</script>

<template>
  <div class="ssl-certificates-view">
    <div class="page-header">
      <div class="header-content">
        <div class="header-title">
          <h1>SSL Certificates</h1>
          <p>Manage SSL certificates for secure connections</p>
        </div>
        <div class="header-actions">
          <Button 
            icon="pi pi-plus" 
            label="Add Certificate" 
            @click="showComingSoon"
            severity="success"
          />
        </div>
      </div>
    </div>

    <div class="content-container">
      <Card class="coming-soon-card">
        <template #content>
          <div class="coming-soon-content">
            <div class="icon-wrapper">
              <i class="pi pi-shield" aria-hidden="true"></i>
            </div>
            <h2>SSL Certificate Management</h2>
            <p>This feature is currently in development and will be available soon.</p>
            <p>You'll be able to:</p>
            <ul class="feature-list">
              <li><i class="pi pi-check"></i> Upload SSL certificates</li>
              <li><i class="pi pi-check"></i> Manage certificate renewals</li>
              <li><i class="pi pi-check"></i> View certificate expiration dates</li>
              <li><i class="pi pi-check"></i> Auto-configure HTTPS for hosts</li>
            </ul>
            <Button 
              label="Notify Me When Available" 
              @click="showComingSoon"
              class="notify-btn"
              outlined
            />
          </div>
        </template>
      </Card>
    </div>
  </div>
</template>

<style scoped>
.ssl-certificates-view {
  padding: 0;
}

.page-header {
  background: var(--surface-card);
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
  border: 1px solid var(--surface-border);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 2rem;
}

.header-title h1 {
  margin: 0 0 0.5rem 0;
  color: var(--text-color);
  font-size: 2rem;
  font-weight: 700;
}

.header-title p {
  margin: 0;
  color: var(--text-color-secondary);
  font-size: 1.1rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
  flex-shrink: 0;
}

.content-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60vh;
}

.coming-soon-card {
  max-width: 600px;
  width: 100%;
  text-align: center;
}

.coming-soon-content {
  padding: 3rem 2rem;
}

.icon-wrapper {
  margin-bottom: 2rem;
}

.icon-wrapper i {
  font-size: 4rem;
  color: var(--primary-color);
  opacity: 0.8;
}

.coming-soon-content h2 {
  margin: 0 0 1rem 0;
  color: var(--text-color);
  font-size: 1.75rem;
  font-weight: 600;
}

.coming-soon-content p {
  margin: 0 0 1.5rem 0;
  color: var(--text-color-secondary);
  font-size: 1.1rem;
  line-height: 1.6;
}

.feature-list {
  list-style: none;
  padding: 0;
  margin: 2rem 0;
  text-align: left;
  max-width: 400px;
  margin-left: auto;
  margin-right: auto;
}

.feature-list li {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
  color: var(--text-color-secondary);
}

.feature-list li i {
  color: var(--green-500);
  font-size: 0.875rem;
}

.notify-btn {
  margin-top: 1.5rem;
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: stretch;
  }

  .header-title h1 {
    font-size: 1.75rem;
  }

  .header-actions {
    justify-content: stretch;
  }

  .header-actions :deep(.p-button) {
    justify-content: center;
  }

  .coming-soon-content {
    padding: 2rem 1rem;
  }

  .icon-wrapper i {
    font-size: 3rem;
  }

  .coming-soon-content h2 {
    font-size: 1.5rem;
  }
}
</style> 