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
          <label for="cost-price" class="block text-sm font-medium text-gray-700 mb-1">Cost Price (in Pcs) *</label>
          <input type="number" id="cost-price" v-model.number="newProduct.cost_price" required placeholder="Enter cost price" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>

        <!-- Selling Price -->
        <div>
          <label for="selling-price" class="block text-sm font-medium text-gray-700 mb-1">Selling Price (in Pcs)*</label>
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

         <div>
          <label for="carton-size" class="block text-sm font-medium text-gray-700 mb-1">Carton Size Name *</label>
          <input type="number" id="carton_size" v-model="newProduct.carton_size" required placeholder="Enter Carton Size" class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>


        <!-- Submit Button -->
        <button type="submit" class="mt-2 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700">
          Add Product
        </button>
      </form>
    </div>

    <section class="bg-white p-4 rounded-lg shadow mb-6">
  <h2 class="text-lg font-semibold text-gray-700 mb-2">📤 Upload Products CSV</h2>
  <input
    type="file"
    accept=".csv"
    @change="handleCsvUpload"
    class="block w-full border border-gray-300 rounded p-2"
  />
</section>

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
import Papa from 'papaparse';

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
      carton_size: 1,
    };
  } catch (error) {
    console.error('Add failed:', error);
    alert('Error saving product.');
  }
}

async function handleCsvUpload(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = async (e) => {
    const text = typeof e.target?.result === 'string' ? e.target.result : '';
if (!text) {
  alert('❌ Unable to read file contents.');
  return;
}
    const lines = text.split('\n').filter(line => line.trim());
    const headers = lines[0].split(',').map(h => h.trim().toLowerCase());

    let successCount = 0;
    let failureCount = 0;

    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(',').map(v => v.trim());
      if (values.length < 5) {
        console.warn(`❌ Row ${i + 1} skipped: Not enough columns`);
        failureCount++;
        continue;
      }

      const row: Record<string, string> = {};
      headers.forEach((key, idx) => row[key] = values[idx]);

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
        console.warn(`⚠️ Row ${i + 1}: Missing or invalid data`);
        failureCount++;
        continue;
      }

      try {
        await apiAddProduct(product);
        successCount++;
      } catch (error) {
        console.error(`❌ Row ${i + 1} failed:`, error);
        failureCount++;
      }
    }

    alert(`✅ Upload complete: ${successCount} added, ${failureCount} failed.`);
    await loadProducts();
  };

  reader.readAsText(file);
}



onMounted(() => {
  loadProducts();
});
</script>
