<template>
    <div class="p-6">
      <h1 class="text-2xl font-bold mb-4">Product Details</h1>
  
      <div v-if="product">
        <p><strong>Name:</strong> {{ product.name }}</p>
        <p><strong>Cost Price:</strong> {{ product.costPrice }}</p>
        <p><strong>Selling Price:</strong> {{ product.sellingPrice }}</p>
        <p><strong>Barcode:</strong> {{ product.barcode || 'N/A' }}</p>
        <p><strong>Description:</strong> {{ product.description || 'N/A' }}</p>
      </div>
  
      <div v-else class="text-red-500 mb-4">
        No product selected.
      </div>
  
      <!-- Back Button -->
      <button
        @click="goBack"
        class="mt-6 inline-flex items-center px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700"
      >
        ← Back to Products
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { useRouter } from 'vue-router';
  import { sharedData } from '@/store/shared';
  
  const product = sharedData.selectedProduct;
  const router = useRouter();
  
  function goBack() {
    router.push('/products');
  }
  </script>
  