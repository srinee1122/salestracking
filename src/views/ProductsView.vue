<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">Product Management</h1>

    <!-- Add New Product Form -->
    <div class="bg-white p-6 rounded-lg shadow-md mb-8">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Add New Product</h2>

      <form @submit.prevent="handleAddProduct" class="space-y-4">
        <!-- Product Name -->
        <div>
          <label for="product-name" class="block text-sm font-medium text-gray-700 mb-1">Product Name *</label>
          <input type="text" id="product-name" v-model="newProduct.name" required placeholder="Enter product name" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Brand -->
        <div>
          <label for="product-brand" class="block text-sm font-medium text-gray-700 mb-1">Brand *</label>
          <input type="text" id="product-brand" v-model="newProduct.brand" required placeholder="Enter product brand" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Cost Price -->
        <div>
          <label for="cost-price" class="block text-sm font-medium text-gray-700 mb-1">Cost Price *</label>
          <input type="number" id="cost-price" v-model.number="newProduct.cost_price" required placeholder="Enter cost price" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Selling Price -->
        <div>
          <label for="selling-price" class="block text-sm font-medium text-gray-700 mb-1">Selling Price *</label>
          <input type="number" id="selling-price" v-model.number="newProduct.unit_price" required placeholder="Enter selling price" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Barcode -->
        <div>
          <label for="barcode" class="block text-sm font-medium text-gray-700 mb-1">Barcode</label>
          <input type="text" id="barcode" v-model="newProduct.sku" placeholder="Enter barcode (optional)" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Description -->
        <div>
          <label for="description" class="block text-sm font-medium text-gray-700 mb-1">Product Description</label>
          <textarea id="description" v-model="newProduct.description" placeholder="Enter description (optional)" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm"></textarea>
        </div>

        <!-- Submit Button -->
        <button type="submit" class="mt-2 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700">
          Add Product
        </button>
      </form>
    </div>

    <!-- Product List Table -->
    <div class="bg-white p-6 rounded-lg shadow-md">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Product List</h2>

      <table v-if="products.length > 0" class="w-full table-auto border-collapse border">
        <thead>
          <tr>
            <th class="border px-3 py-2 text-left">Name</th>
            <th class="border px-3 py-2 text-left">Brand</th>
            <th class="border px-3 py-2 text-left">Cost Price</th>
            <th class="border px-3 py-2 text-left">Selling Price</th>
            <th class="border px-3 py-2 text-left">Barcode</th>
            <th class="border px-3 py-2 text-left">Description</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(product, index) in products" :key="index">
            <td class="border px-3 py-2">{{ product.name }}</td>
            <td class="border px-3 py-2">{{ product.brand }}</td>
            <td class="border px-3 py-2">{{ product.cost_price }}</td>
            <td class="border px-3 py-2">{{ product.unit_price }}</td>
            <td class="border px-3 py-2">{{ product.sku || '-' }}</td>
            <td class="border px-3 py-2">{{ product.description || '-' }}</td>
          </tr>
        </tbody>
      </table>

      <p v-else class="text-gray-500">No products added yet.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { apiAddProduct, apiFetchProducts, ProductPayload, Product } from '../model/products';

const newProduct = ref<ProductPayload>({
  sku: '',
  name: '',
  brand: '',
  category: 'General',
  cost_price: 0,
  unit_price: 0,
  description: '',
});

const products = ref<Product[]>([]);
const isLoading = ref(false);

async function loadProducts() {
  isLoading.value = true;
  try {
    products.value = await apiFetchProducts();
  } catch (error) {
    console.error('Failed to fetch products:', error);
    alert('Could not load products from database');
  } finally {
    isLoading.value = false;
  }
}

async function handleAddProduct() {
  if (!newProduct.value.name || !newProduct.value.brand || newProduct.value.cost_price <= 0 || newProduct.value.unit_price <= 0) {
    alert('Product name, brand, and price are required.');
    return;
  }

  try {
    await apiAddProduct(newProduct.value);
    alert('Product added successfully!');
    await loadProducts();
    newProduct.value = {
      sku: '',
      name: '',
      brand: '',
      category: 'General',
      cost_price: 0,
      unit_price: 0,
      description: '',
    };
  } catch (error) {
    console.error('Add failed:', error);
    alert('Error saving product.');
  }
}

onMounted(() => {
  loadProducts();
});
</script>
