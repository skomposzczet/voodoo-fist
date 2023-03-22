import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import DashboardPage from '../views/DashboardPage.vue'
import AboutPage from '../views/AboutPage.vue'
import AuthPage from '../views/AuthPage.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: DashboardPage,
  },
  {
    path: '/about',
    name: 'about',
    component: AboutPage,
  },
  {
    path: '/auth',
    name: 'auth',
    component: AuthPage,
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
