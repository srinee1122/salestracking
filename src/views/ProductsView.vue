<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">Product Management</h1>

     <!-- Search bar -->
    <div class="mb-4">
  <input
    v-model="searchQuery"
    placeholder="🔍 Search by name, brand, ID..."
    class="w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm"
  />
</div>

   <!-- Product List -->


  <!-- Product List: Vertical Layout with Scroll -->
<div class="bg-white p-6 rounded-lg shadow-md">
  <h2 class="text-xl font-semibold text-gray-700 mb-4">Product List</h2>

  <div class="overflow-y-auto max-h-[500px] border rounded">
    <table class="w-full table-auto border-collapse">
      <thead class="sticky top-0 bg-gray-100 z-10">
        <tr>
          <th class="border px-3 py-2 text-left">SL NO</th>
          <th class="border px-3 py-2 text-left">Name</th>
          <th class="border px-3 py-2 text-left">Brand</th>
          <th class="border px-3 py-2 text-left">Cost Price</th>
          <th class="border px-3 py-2 text-left">Selling Price</th>
          <th class="border px-3 py-2 text-left">Barcode</th>
          <th class="border px-3 py-2 text-left">Description</th>
          <th class="border px-3 py-2 text-left">Carton Size</th>
          <th class="border px-3 py-2 text-left">Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(product, index) in filteredProducts"
          :key="product.id"
          :class="[
            isEditMode && newProduct.id === product.id ? 'bg-yellow-100' : '',
            index % 2 === 0 ? 'bg-white' : 'bg-gray-100',
            'hover:bg-blue-50 transition-colors'
          ]"
        >
          <td class="border px-3 py-2">{{ index }}</td>
          <td class="border px-3 py-2">{{ product.name }}</td>
          <td class="border px-3 py-2">{{ product.brand }}</td>
          <td class="border px-3 py-2">{{ product.cost_price }}</td>
          <td class="border px-3 py-2">{{ product.unit_price }}</td>
          <td class="border px-3 py-2">{{ product.sku }}</td>
          <td class="border px-3 py-2">{{ product.description }}</td>
          <td class="border px-3 py-2">{{ product.carton_size }}</td>
          <td class="border px-3 py-2 space-x-2">
            <button @click="editProduct(product)" class="text-blue-600 hover:underline text-sm">Edit</button>
            <button @click="deleteProduct(product.id)" class="text-red-600 hover:underline text-sm">Delete</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</div>

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
    <!-- <div class="mb-4">
      <input v-model="searchQuery" placeholder="Search product..." class="w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
    </div> -->

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
          <input   type="number" step="0.01" inputmode="decimal" v-model="newProduct.cost_price" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Selling Price (in Pcs)*</label>
          <input   type="number" step="0.01" inputmode="decimal" v-model="newProduct.unit_price" required class="block w-full px-4 py-2 border rounded-md shadow-sm sm:text-sm" />
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
            <tr
  v-for="(item, index) in csvPreview"
  :key="index"
  :class="[
    csvErrors[index] ? 'bg-red-100' : 'bg-green-50',
    'transition-colors'
  ]"
>+
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

   
</div>
</template>

<script setup lang="ts">
import { ref, onMounted,computed } from 'vue';
import { apiAddProduct, apiFetchProducts, apiUpdateProduct, apiDeleteProduct } from '@/model/products';
import Papa from 'papaparse';
import { invoke } from '@tauri-apps/api';

const products = ref([]);
const newProduct = ref({ id: null, sku: '', name: '', brand: '', category: 'General', cost_price: 0, unit_price: 0, description: '', carton_size: 1 });
const isEditMode = ref(false);
const csvPreview = ref([]);
const searchQuery = ref('');

function cancelEdit() {
  isEditMode.value = false;
  newProduct.value = { id: null, sku: '', name: '', brand: '', category: 'General', cost_price: 0, unit_price: 0, description: '', carton_size: 1 };
}


function editProduct(product: any) {
  newProduct.value = { ...product };
  isEditMode.value = true;
}

async function deleteProduct(productId: number) {
  const confirmDelete = confirm("Are you sure you want to delete this product?");
  if (!confirmDelete) return;

 try {
  await apiDeleteProduct(productId);
  alert("✅ Product deleted.");
  await loadProducts();
} catch (err) {
  showDeleteError(err);
}
}
async function downloadTemplate() {
  const content = `name,brand,sku,cost_price,unit_price,description,carton_size,category\nSample Product,Sample Brand,123456,1.99,2.99,Sample Description,24,General`;
  const blob = new Blob([content], { type: 'text/csv' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = 'sample-product-template.csv';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}


const csvErrors = ref<string[]>([]); // Holds error messages per row
function handleCsvPreview(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  Papa.parse(file, {
    header: true,
    skipEmptyLines: true,
    complete: (results) => {
      const parsed = (results.data as any[]).map((row, index) => {
        const parsedRow = {
          name: row.name || '',
          brand: row.brand || '',
          sku: row.sku || '',
          cost_price: parseFloat(row.cost_price),
          unit_price: parseFloat(row.unit_price),
          description: row.description || '',
          carton_size: parseInt(row.carton_size || '1'),
          category: row.category || 'General'
        };

        // Validate row
        const isValid = parsedRow.name && parsedRow.brand && parsedRow.cost_price > 0 && parsedRow.unit_price > 0;
        csvErrors.value[index] = isValid ? '' : 'Invalid row';
        return parsedRow;
      });

      csvPreview.value = parsed;
    }
  });
}
async function confirmCsvUpload() {
  let success = 0, failed = 0;

  for (let i = 0; i < csvPreview.value.length; i++) {
    const row = csvPreview.value[i];
    if (csvErrors.value[i]) {
      failed++;
      continue;
    }

    try {
      await apiAddProduct(row);
      success++;
    } catch (err) {
      console.error(`Upload error (row ${i + 1}):`, err);
      failed++;
    }
  }

  alert(`✅ Upload complete: ${success} added, ${failed} failed.`);
  await loadProducts();
  csvPreview.value = [];
  csvErrors.value = [];
}

async function handleAddOrUpdateProduct() {
  if (!newProduct.value.name || !newProduct.value.brand || newProduct.value.cost_price <= 0 || newProduct.value.unit_price <= 0) {
    alert('⚠️ Name, brand, cost and selling price are required.');
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
    console.error('API Error:', err);
    alert('❌ Save failed. Check if SKU is unique.');
  }
}

function showDeleteError(err: any) {
  const message = typeof err === "string" ? err : err?.message || "❌ Delete failed.";
  alert(message);
}

const filteredProducts = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return products.value.filter(p =>
    p.name.toLowerCase().includes(query) ||
    p.brand.toLowerCase().includes(query) ||
    p.description.toLowerCase().includes(query) ||
    String(p.product_id).includes(query) ||
    String(p.sku).includes(query)
  );
});

async function loadProducts() {
  products.value = await apiFetchProducts();
}

onMounted(() => {
  loadProducts();
});
</script>
