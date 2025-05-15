<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">Product Management</h1>

    <!-- Download Template Button -->
    <div class="mb-4">
     <button
  @click="downloadTemplate"
  class="inline-block bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
>
  ⬇️ Download CSV Template
</button>
    </div>

    <!-- Search Product -->
    <div class="mb-4">
      <input v-model="searchQuery" placeholder="Search product..." class="w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
    </div>

    <!-- Add or Edit Product Form -->
    <div :class="[isEditMode ? 'bg-yellow-100' : 'bg-white', 'p-6 rounded-lg shadow-md mb-8']">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">{{ isEditMode ? 'Edit Product' : 'Add New Product' }}</h2>
      <form @submit.prevent="handleAddOrUpdateProduct" class="space-y-4">
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
        <div class="flex items-center gap-3">
          <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700">
            {{ isEditMode ? 'Update Product' : 'Add Product' }}
          </button>
          <button v-if="isEditMode" @click="cancelEdit" type="button" class="py-2 px-4 text-sm font-medium rounded-md bg-gray-300 hover:bg-gray-400">
            Cancel Edit
          </button>
        </div>
      </form>
    </div>

    <!-- CSV Upload and Preview -->
    <section class="bg-white p-4 rounded-lg shadow mb-6">
      <h2 class="text-lg font-semibold text-gray-700 mb-2">📤 Upload Products CSV</h2>
      <input type="file" accept=".csv" @change="handleCsvPreview" class="block w-full border border-gray-300 rounded p-2" />

      <div v-if="csvPreview.length" class="mt-4">
        <h3 class="font-semibold text-gray-700 mb-2">Preview:</h3>
        <table class="w-full table-auto border">
          <thead>
            <tr>
              <th class="border px-3 py-1">Name</th>
              <th class="border px-3 py-1">Brand</th>
              <th class="border px-3 py-1">Cost Price</th>
              <th class="border px-3 py-1">Unit Price</th>
              <th class="border px-3 py-1">Carton Size</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item, index) in csvPreview" :key="index">
              <td class="border px-3 py-1" :class="{ 'text-red-600': !item.name }">{{ item.name }}</td>
              <td class="border px-3 py-1" :class="{ 'text-red-600': !item.brand }">{{ item.brand }}</td>
              <td class="border px-3 py-1" :class="{ 'text-red-600': item.cost_price <= 0 }">{{ item.cost_price }}</td>
              <td class="border px-3 py-1" :class="{ 'text-red-600': item.unit_price <= 0 }">{{ item.unit_price }}</td>
              <td class="border px-3 py-1">{{ item.carton_size }}</td>
            </tr>
          </tbody>
        </table>
        <button class="mt-4 bg-blue-600 text-white px-4 py-2 rounded" @click="confirmCsvUpload">✅ Confirm & Upload</button>
      </div>
    </section>

    <!-- Product List -->
    <div class="bg-white p-6 rounded-lg shadow-md">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">Product List</h2>
      <table v-if="filteredProducts.length > 0" class="w-full table-auto border-collapse border">
        <thead>
          <tr>
            <th class="border px-3 py-2 text-left">Name</th>
            <th class="border px-3 py-2 text-left">Brand</th>
            <th class="border px-3 py-2 text-left">Cost Price</th>
            <th class="border px-3 py-2 text-left">Selling Price</th>
            <th class="border px-3 py-2 text-left">Barcode</th>
            <th class="border px-3 py-2 text-left">Description</th>
            <th class="border px-3 py-2 text-left">Carton Size</th>
            <th class="border px-3 py-2 text-left">Action</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(product, index) in filteredProducts"
            :key="index"
            :class="{ 'bg-yellow-100': isEditMode && newProduct.id === product.id }"
          >
            <td class="border px-3 py-2">{{ product.name }}</td>
            <td class="border px-3 py-2">{{ product.brand }}</td>
            <td class="border px-3 py-2">{{ product.cost_price }}</td>
            <td class="border px-3 py-2">{{ product.unit_price }}</td>
            <td class="border px-3 py-2">{{ product.sku || '-' }}</td>
            <td class="border px-3 py-2">{{ product.description || '-' }}</td>
            <td class="border px-3 py-2">{{ product.carton_size || '-' }}</td>
            <td class="border px-3 py-2">
              <button @click="editProduct(product)" class="text-blue-600 hover:underline">Edit</button>
              <button @click="() => handleDelete(product.id)" class="text-red-600 hover:underline">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-else class="text-gray-500">No products found.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { apiAddProduct, apiFetchProducts, apiUpdateProduct,apiDeleteProduct } from '@/model/products';
import Papa from 'papaparse';

const newProduct = ref({ sku: '', name: '', brand: '', category: 'General', cost_price: 0, unit_price: 0, description: '', carton_size: 1 });
const products = ref([]);
const isEditMode = ref(false);
const csvPreview = ref([]);
const searchQuery = ref('');

const filteredProducts = computed(() => {
  return products.value.filter(p =>
    p.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    p.brand.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

function editProduct(product: any) {
  newProduct.value = { ...product };
  isEditMode.value = true;
}

function cancelEdit() {
  isEditMode.value = false;
  newProduct.value = { sku: '', name: '', brand: '', category: 'General', cost_price: 0, unit_price: 0, description: '', carton_size: 1 };
}

async function handleAddOrUpdateProduct() {
  if (!newProduct.value.name || !newProduct.value.brand || newProduct.value.cost_price <= 0 || newProduct.value.unit_price <= 0) {
    alert('Product name, brand, and price are required.');
    return;
  }

  try {
    if (isEditMode.value && newProduct.value.id) {
      await apiUpdateProduct(newProduct.value);
      alert('✅ Product updated!');
    } else {
      await apiAddProduct(newProduct.value);
      alert('✅ Product added!');
    }
    await loadProducts();
    cancelEdit();
  } catch (err) {
    console.error('Error saving product:', err);
    alert('❌ Failed to save product.');
  }
}


function downloadTemplate() {
  const headers = [
    'name,brand,sku,cost_price,unit_price,description,carton_size,category'
  ];
  const sample = [
    'Sample Product,BrandX,SKU123,10.5,15.0,A sample product,12,General'
  ];
  const csvContent = headers.concat(sample).join('\n');

  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.setAttribute('download', 'sample-product-template.csv');
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

async function handleCsvPreview(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  Papa.parse(file, {
    header: true,
    skipEmptyLines: true,
    complete: (results) => {
      csvPreview.value = (results.data as any[]).map(row => ({
        name: row.name,
        brand: row.brand,
        sku: row.sku,
        cost_price: parseFloat(row.cost_price),
        unit_price: parseFloat(row.unit_price),
        description: row.description || '',
        carton_size: parseInt(row.carton_size || '1'),
        category: row.category || 'General'
      }));
    }
  });
}




async function confirmCsvUpload() {
  let success = 0, failed = 0;
  for (const row of csvPreview.value) {
    if (!row.name || !row.brand || isNaN(row.cost_price) || isNaN(row.unit_price)) {
      failed++;
      continue;
    }
    try {
      await apiAddProduct(row);
      success++;
    } catch (err) {
      console.error('Upload error:', err);
      failed++;
    }
  }
  alert(`✅ Upload complete: ${success} added, ${failed} failed.`);
  await loadProducts();
  csvPreview.value = [];
}

async function handleDelete(productId: number) {
  if (!confirm('Are you sure you want to delete this product?')) return;
  try {
    await apiDeleteProduct(productId);
    alert('✅ Product deleted!');
    await loadProducts();
  } catch (err) {
    console.error('Error deleting product:', err);
    alert('❌ Failed to delete product.');
  }
}

async function loadProducts() {
  products.value = await apiFetchProducts();
}

onMounted(() => {
  loadProducts();
});
</script>
