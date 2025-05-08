// src/router/index.ts

import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';

// Import view components
import SalespersonView from '../views/SalespersonView.vue';
import ProductsView from '../views/ProductsView.vue';
import ProductDetailView from '../views/ProductDetailView.vue';  
import SalesEntryView from '../views/SalesentryView.vue';
import TargetCampaignView from '../views/TargetCampaignView.vue';
import CampaignProgressView from '../views/CampaignProgressView.vue';

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
    path: '/productdetail',
    name: 'ProductDetail',
    component: ProductDetailView // ✅ New route added
  },
  {
    path: '/saleentry',
    name: 'SaleEntry',
    component: SalesEntryView
  }
  ,
  {
    path: '/targetcampaign',
    name: 'targetcampaign',
    component: TargetCampaignView
  },
  {
    path: '/campaign/:id',
    name: 'CampaignProgress',
    component: CampaignProgressView
  }
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
