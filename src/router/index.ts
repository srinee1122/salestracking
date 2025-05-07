// src/router/index.ts

import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';

// Import view components
import SalespersonView from '../views/SalespersonView.vue';
import ProductsView from '../views/ProductsView.vue';
import ProductDetailView from '../views/ProductDetailView.vue'; // 

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Salespeople',
    component: SalespersonView
  },
  {
    path: '/products',
    name: 'Products',
    component: ProductsView
  },
  {
    path: '/product-detail',
    name: 'ProductDetail',
    component: ProductDetailView // ✅ New route added
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
