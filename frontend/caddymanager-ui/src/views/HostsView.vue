<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Dropdown from 'primevue/dropdown'
import Tag from 'primevue/tag'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'

interface Host {
  name: string
  ip: string
  port: number
  scheme: string
  status?: string
}

const toast = useToast()
const confirm = useConfirm()
const hosts = ref<Host[]>([])
const dialogVisible = ref(false)
const globalFilterValue = ref('')
const newHost = ref({
  name: '',
  ip: '',
  port: '80',
  scheme: 'http',
})

const schemeOptions = [
  { label: 'HTTP', value: 'http' },
  { label: 'HTTPS', value: 'https' },
]

const API_URL = 'http://localhost:8080/api'

const filteredHosts = computed(() => {
  if (!globalFilterValue.value) return hosts.value

  return hosts.value.filter(
    (host) =>
      host.name.toLowerCase().includes(globalFilterValue.value.toLowerCase()) ||
      host.ip.toLowerCase().includes(globalFilterValue.value.toLowerCase()) ||
      host.scheme.toLowerCase().includes(globalFilterValue.value.toLowerCase()),
  )
})

const fetchHosts = async () => {
  try {
    const response = await fetch(`${API_URL}/hosts`)
    if (!response.ok) throw new Error('Failed to fetch hosts')
    const data = await response.json()
    hosts.value = data.map((host: any) => ({
      ...host,
      status: 'online', // Mock status for demo
    }))
  } catch (error: any) {
    toast.add({ severity: 'error', summary: 'Error', detail: error.message, life: 3000 })
  }
}

onMounted(fetchHosts)

const openDialog = () => {
  newHost.value = { name: '', ip: '', port: '80', scheme: 'http' }
  dialogVisible.value = true
}

const closeDialog = () => {
  dialogVisible.value = false
}

const validateHost = () => {
  if (!newHost.value.name.trim()) {
    toast.add({
      severity: 'error',
      summary: 'Validation Error',
      detail: 'Host name is required',
      life: 3000,
    })
    return false
  }
  if (!newHost.value.ip.trim()) {
    toast.add({
      severity: 'error',
      summary: 'Validation Error',
      detail: 'IP address is required',
      life: 3000,
    })
    return false
  }
  const port = parseInt(newHost.value.port)
  if (isNaN(port) || port < 1 || port > 65535) {
    toast.add({
      severity: 'error',
      summary: 'Validation Error',
      detail: 'Port must be between 1 and 65535',
      life: 3000,
    })
    return false
  }
  return true
}

const addHost = async () => {
  if (!validateHost()) return

  try {
    const response = await fetch(`${API_URL}/hosts`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        ...newHost.value,
        port: parseInt(newHost.value.port)
      }),
    })
    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(errorText || 'Failed to add host')
    }
    toast.add({
      severity: 'success',
      summary: 'Success',
      detail: 'Host added successfully',
      life: 3000,
    })
    fetchHosts()
    closeDialog()
  } catch (error: any) {
    toast.add({ severity: 'error', summary: 'Error', detail: error.message, life: 3000 })
  }
}

const confirmDelete = (host: any) => {
  confirm.require({
    message: `Are you sure you want to delete host "${host.name}"?`,
    header: 'Confirm Deletion',
    icon: 'pi pi-exclamation-triangle',
    rejectProps: {
      label: 'Cancel',
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      label: 'Delete',
      severity: 'danger',
    },
    accept: () => {
      deleteHost(host.name)
    },
  })
}

const deleteHost = async (name: string) => {
  try {
    const response = await fetch(`${API_URL}/hosts/${name}`, {
      method: 'DELETE',
    })
    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(errorText || 'Failed to delete host')
    }
    toast.add({
      severity: 'success',
      summary: 'Success',
      detail: 'Host deleted successfully',
      life: 3000,
    })
    fetchHosts()
  } catch (error: any) {
    toast.add({ severity: 'error', summary: 'Error', detail: error.message, life: 3000 })
  }
}

const getStatusSeverity = (status: string) => {
  switch (status) {
    case 'online':
      return 'success'
    case 'offline':
      return 'danger'
    case 'maintenance':
      return 'warning'
    default:
      return 'info'
  }
}

const refreshHosts = () => {
  fetchHosts()
  toast.add({
    severity: 'info',
    summary: 'Refreshed',
    detail: 'Host list updated',
    life: 2000,
  })
}
</script>

<template>
  <div class="hosts-view">
    <div class="page-header">
      <div class="header-content">
        <div class="header-title">
          <h1>Hosts</h1>
          <p>Configure and manage your backend servers</p>
        </div>
        <div class="header-actions">
          <Button
            label="Refresh"
            icon="pi pi-refresh"
            @click="refreshHosts"
            severity="secondary"
            outlined
          />
          <Button label="Add Host" icon="pi pi-plus" @click="openDialog" />
        </div>
      </div>
    </div>

    <div class="hosts-card">
      <div class="card-toolbar">
        <div class="search-section">
          <div class="search-input">
            <i class="pi pi-search search-icon"></i>
            <InputText
              v-model="globalFilterValue"
              placeholder="Search hosts..."
              class="search-field"
            />
          </div>
        </div>
        <div class="toolbar-info">
          <span class="host-count"
            >{{ filteredHosts.length }} host{{ filteredHosts.length !== 1 ? 's' : '' }}</span
          >
        </div>
      </div>

      <DataTable
        :value="filteredHosts"
        :paginator="true"
        :rows="10"
        :rowsPerPageOptions="[5, 10, 25, 50]"
        paginatorTemplate="RowsPerPageDropdown FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
        currentPageReportTemplate="Showing {first} to {last} of {totalRecords} entries"
        class="hosts-table"
        stripedRows
        showGridlines
        :metaKeySelection="false"
      >
        <template #empty>
          <div class="empty-state">
            <i class="pi pi-server empty-icon"></i>
            <h3>No hosts configured</h3>
            <p>Get started by adding your first backend server</p>
            <Button label="Add Host" icon="pi pi-plus" @click="openDialog" class="empty-action" />
          </div>
        </template>

        <Column field="name" header="Host Name" sortable>
          <template #body="slotProps">
            <div class="host-name">
              <i class="pi pi-server host-icon"></i>
              <span class="name">{{ slotProps.data.name }}</span>
            </div>
          </template>
        </Column>

        <Column field="ip" header="IP Address" sortable>
          <template #body="slotProps">
            <code class="ip-address">{{ slotProps.data.ip }}</code>
          </template>
        </Column>

        <Column field="port" header="Port" sortable>
          <template #body="slotProps">
            <Tag :value="slotProps.data.port" severity="info" class="port-tag" />
          </template>
        </Column>

        <Column field="scheme" header="Protocol" sortable>
          <template #body="slotProps">
            <Tag
              :value="slotProps.data.scheme.toUpperCase()"
              :severity="slotProps.data.scheme === 'https' ? 'success' : 'secondary'"
              :icon="slotProps.data.scheme === 'https' ? 'pi pi-lock' : 'pi pi-globe'"
            />
          </template>
        </Column>

        <Column field="status" header="Status" sortable>
          <template #body="slotProps">
            <Tag
              :value="slotProps.data.status"
              :severity="getStatusSeverity(slotProps.data.status)"
              :icon="slotProps.data.status === 'online' ? 'pi pi-check' : 'pi pi-times'"
            />
          </template>
        </Column>

        <Column header="Actions">
          <template #body="slotProps">
            <div class="action-buttons">
              <Button
                icon="pi pi-eye"
                severity="info"
                outlined
                rounded
                size="small"
                @click="
                  toast.add({
                    severity: 'info',
                    summary: 'Info',
                    detail: 'View functionality coming soon!',
                    life: 3000,
                  })
                "
                title="View Details"
              />
              <Button
                icon="pi pi-pencil"
                severity="warning"
                outlined
                rounded
                size="small"
                @click="
                  toast.add({
                    severity: 'info',
                    summary: 'Info',
                    detail: 'Edit functionality coming soon!',
                    life: 3000,
                  })
                "
                title="Edit Host"
              />
              <Button
                icon="pi pi-trash"
                severity="danger"
                outlined
                rounded
                size="small"
                @click="confirmDelete(slotProps.data)"
                title="Delete Host"
              />
            </div>
          </template>
        </Column>
      </DataTable>
    </div>

    <Dialog
      header="Add New Host"
      v-model:visible="dialogVisible"
      :modal="true"
      @hide="closeDialog"
      class="host-dialog"
      :style="{ width: '500px' }"
    >
      <template #header>
        <div class="dialog-header">
          <i class="pi pi-server dialog-icon"></i>
          <span>Add New Host</span>
        </div>
      </template>

      <div class="host-form">
        <div class="form-field">
          <label for="name" class="field-label">
            <i class="pi pi-tag"></i>
            Host Name *
          </label>
          <InputText
            id="name"
            v-model="newHost.name"
            placeholder="e.g., web-server-01"
            class="form-input"
          />
        </div>

        <div class="form-field">
          <label for="ip" class="field-label">
            <i class="pi pi-globe"></i>
            IP Address *
          </label>
          <InputText
            id="ip"
            v-model="newHost.ip"
            placeholder="e.g., 192.168.1.100"
            class="form-input"
          />
        </div>

        <div class="form-row">
          <div class="form-field">
            <label for="port" class="field-label">
              <i class="pi pi-link"></i>
              Port *
            </label>
            <InputText
              id="port"
              v-model.number="newHost.port"
              type="number"
              min="1"
              max="65535"
              placeholder="80"
              class="form-input"
            />
          </div>

          <div class="form-field">
            <label for="scheme" class="field-label">
              <i class="pi pi-shield"></i>
              Protocol
            </label>
            <Dropdown
              id="scheme"
              v-model="newHost.scheme"
              :options="schemeOptions"
              optionLabel="label"
              optionValue="value"
              placeholder="Select protocol"
              class="form-input"
            />
          </div>
        </div>
      </div>

      <template #footer>
        <div class="dialog-footer">
          <Button
            label="Cancel"
            icon="pi pi-times"
            @click="closeDialog"
            severity="secondary"
            outlined
          />
          <Button
            label="Add Host"
            icon="pi pi-check"
            @click="addHost"
            :disabled="!newHost.name || !newHost.ip"
          />
        </div>
      </template>
    </Dialog>
  </div>
</template>

<style scoped>
.hosts-view {
  padding: 0;
}

.page-header {
  background: linear-gradient(135deg, var(--green-50) 0%, var(--teal-50) 100%);
  border-radius: 1rem;
  padding: 2rem;
  margin-bottom: 2rem;
  border: 1px solid var(--surface-border);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.header-title h1 {
  margin: 0 0 0.5rem 0;
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--text-color);
}

.header-title p {
  margin: 0;
  color: var(--text-color-secondary);
  font-size: 1.1rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.hosts-card {
  background: var(--surface-card);
  border-radius: 1rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.card-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem 2rem;
  background: var(--surface-50);
  border-bottom: 1px solid var(--surface-border);
}

.search-section {
  flex: 1;
  max-width: 400px;
}

.search-input {
  position: relative;
  width: 100%;
}

.search-icon {
  position: absolute;
  left: 1rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-color-secondary);
  z-index: 1;
}

.search-field {
  width: 100%;
  padding-left: 3rem;
  border-radius: 2rem;
  border: 1px solid var(--surface-border);
  background: var(--surface-card);
}

.search-field:focus {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 0.1rem var(--primary-color);
}

.toolbar-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.host-count {
  font-size: 0.9rem;
  color: var(--text-color-secondary);
  font-weight: 500;
}

.hosts-table {
  border: none;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-color-secondary);
}

.empty-icon {
  font-size: 4rem;
  color: var(--text-color-secondary);
  margin-bottom: 1rem;
}

.empty-state h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-color);
  font-size: 1.5rem;
}

.empty-state p {
  margin: 0 0 2rem 0;
  font-size: 1rem;
}

.empty-action {
  padding: 0.75rem 2rem;
}

.host-name {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.host-icon {
  color: var(--primary-color);
  font-size: 1.1rem;
}

.name {
  font-weight: 600;
  color: var(--text-color);
}

.ip-address {
  font-family: 'Courier New', monospace;
  background: var(--surface-100);
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.9rem;
  color: var(--text-color);
  border: 1px solid var(--surface-border);
}

.port-tag {
  font-family: 'Courier New', monospace;
  font-weight: 600;
}

.action-buttons {
  display: flex;
  gap: 0.5rem;
  justify-content: center;
  align-items: center;
}

.action-buttons :deep(.p-button) {
  width: 2rem;
  height: 2rem;
}

.action-buttons :deep(.p-button-icon) {
  font-size: 0.875rem;
}

.host-dialog {
  border-radius: 1rem;
  overflow: hidden;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 1.25rem;
  font-weight: 600;
}

.dialog-icon {
  color: var(--primary-color);
  font-size: 1.5rem;
}

.host-form {
  padding: 0.5rem 0;
}

.form-field {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.field-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--text-color);
  font-size: 0.9rem;
}

.field-label i {
  color: var(--primary-color);
  font-size: 0.9rem;
}

.form-input {
  width: 100%;
  border-radius: 0.5rem;
  border: 1px solid var(--surface-border);
  padding: 0.75rem 1rem;
  transition: all 0.2s ease;
}

.form-input:focus {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 0.1rem var(--primary-color);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1rem;
}

@media (max-width: 1024px) {
  .header-content {
    flex-direction: column;
    gap: 1.5rem;
    align-items: stretch;
  }

  .header-actions {
    justify-content: flex-end;
  }

  .card-toolbar {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }

  .search-section {
    max-width: none;
  }
}

@media (max-width: 768px) {
  .header-title h1 {
    font-size: 2rem;
  }

  .header-actions {
    flex-direction: column;
  }

  .form-row {
    grid-template-columns: 1fr;
  }

  .action-buttons {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>
