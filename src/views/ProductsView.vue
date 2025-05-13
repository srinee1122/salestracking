<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">Product Management</h1>

    <!-- Add New Product Form -->
    <div class="bg-white p-6 rounded-lg shadow-md mb-8">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Add New Product</h2>
      <form @submit.prevent="handleAddProduct" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Product Name *</label>
          <input type="text" v-model="newProduct.name" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Brand *</label>
          <input type="text" v-model="newProduct.brand" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Cost Price (in Pcs) *</label>
          <input type="number" v-model.number="newProduct.cost_price" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Selling Price (in Pcs)*</label>
          <input type="number" v-model.number="newProduct.unit_price" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Barcode</label>
          <input type="text" v-model="newProduct.sku" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Product Description</label>
          <textarea v-model="newProduct.description" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm"></textarea>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Carton Size *</label>
          <input type="number" v-model="newProduct.carton_size" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <button type="submit" class="mt-2 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700">
          Add Product
        </button>
      </form>
    </div>

    <!-- Upload Products CSV -->
    <div class="bg-white p-6 rounded-lg shadow-md mb-8">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">📤 Upload Products CSV</h2>
      <input type="file" accept=".csv" @change="previewCsvUpload" class="block w-full border border-gray-300 rounded p-2 mb-4" />

      <div v-if="csvPreview.length">
        <h3 class="text-md font-medium text-gray-800 mb-2">Preview:</h3>
        <table class="w-full table-auto border-collapse border mb-4">
          <thead>
            <tr>
              <th class="border px-3 py-2 text-left">Name</th>
              <th class="border px-3 py-2 text-left">Brand</th>
              <th class="border px-3 py-2 text-left">Cost Price</th>
              <th class="border px-3 py-2 text-left">Selling Price</th>
              <th class="border px-3 py-2 text-left">Barcode</th>
              <th class="border px-3 py-2 text-left">Description</th>
              <th class="border px-3 py-2 text-left">Carton Size</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(product, index) in csvPreview" :key="index">
              <td class="border px-3 py-2">{{ product.name }}</td>
              <td class="border px-3 py-2">{{ product.brand }}</td>
              <td class="border px-3 py-2">{{ product.cost_price }}</td>
              <td class="border px-3 py-2">{{ product.unit_price }}</td>
              <td class="border px-3 py-2">{{ product.sku }}</td>
              <td class="border px-3 py-2">{{ product.description }}</td>
              <td class="border px-3 py-2">{{ product.carton_size }}</td>
            </tr>
          </tbody>
        </table>
        <button @click="uploadConfirmedCsv" class="btn-blue">✅ Confirm & Upload</button>
      </div>
    </div>

    <!-- Product List -->
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
            <th class="border px-3 py-2 text-left">Carton Size</th>
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
            <td class="border px-3 py-2">{{ product.carton_size || '-' }}</td>
          </tr>
        </tbody>
      </table>
      <p v-else class="text-gray-500">No products added yet.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Papa from 'papaparse';
import { apiAddProduct, apiFetchProducts, ProductPayload, Product } from '../model/products';

const newProduct = ref<ProductPayload>({
  sku: '',
  name: '',
  brand: '',
  category: 'General',
  cost_price: 0,
  unit_price: 0,
  description: '',
  carton_size: 1,
});

const products = ref<Product[]>([]);
const csvPreview = ref<any[]>([]);

async function loadProducts() {
  products.value = await apiFetchProducts();
}

async function handleAddProduct() {
  if (!newProduct.value.name || !newProduct.value.brand || newProduct.value.cost_price <= 0 || newProduct.value.unit_price <= 0) {
    alert('Product name, brand, and price are required.');
    return;
  }

  try {
    await apiAddProduct(newProduct.value);
    alert('✅ Product added!');
    await loadProducts();
    newProduct.value = {
      sku: '', name: '', brand: '', category: 'General', cost_price: 0, unit_price: 0, description: '', carton_size: 1
    };
  } catch (err) {
    console.error('Error saving product:', err);
    alert('❌ Failed to save product.');
  }
}

function previewCsvUpload(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;

  Papa.parse(file, {
    header: true,
    skipEmptyLines: true,
    complete(results) {
      csvPreview.value = results.data as any[];
    },
  });
}

async function uploadConfirmedCsv() {
  let success = 0, failed = 0;

  for (const row of csvPreview.value) {
    const product = {
      name: row.name,
      brand: row.brand,
      sku: row.sku,
      cost_price: parseFloat(row.cost_price),
      unit_price: parseFloat(row.unit_price),
      description: row.description || '',
      carton_size: parseInt(row.carton_size || '1'),
      category: row.category || 'General'
    };

    if (!product.name || !product.brand || isNaN(product.cost_price) || isNaN(product.unit_price)) {
      failed++;
      continue;
    }

    try {
      await apiAddProduct(product);
      success++;
    } catch (e) {
      failed++;
      console.error('Upload error:', e);
    }
  }

  alert(`✅ Upload done: ${success} success, ${failed} failed.`);
  csvPreview.value = [];
  await loadProducts();
}

onMounted(loadProducts);
</script>
