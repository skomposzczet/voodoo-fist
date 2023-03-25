import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import DashboardPage from '../views/DashboardPage.vue'
import AboutPage from '../views/AboutPage.vue'
import AuthPage from '../views/AuthPage.vue'
import ListPage from '../views/ListPage.vue'
import NotFound from '../views/NotFound.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: DashboardPage,
  },
  {
    path: '/list/:id',
    name: 'list',
    component: ListPage,
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
  {
    path: '/:catchAll(.*)',
    name: '404',
    component: NotFound,
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
