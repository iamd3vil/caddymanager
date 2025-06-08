import { createRouter, createWebHistory } from 'vue-router'
import HostsView from '../views/HostsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'hosts',
      component: HostsView
    },
    {
      path: '/ssl-certificates',
      name: 'ssl-certificates',
      component: () => import('../views/SSLCertificatesView.vue')
    },
    // Redirect old routes
    {
      path: '/dashboard',
      redirect: '/'
    },
    {
      path: '/hosts',
      redirect: '/'
    },
    {
      path: '/config',
      redirect: '/'
    },
    {
      path: '/logs',
      redirect: '/'
    },
    {
      path: '/stats',
      redirect: '/'
    }
  ]
})

export default router
