<template>
    <div class="p-6 sm:p-8">
      <h1 class="text-2xl font-bold text-gray-800 mb-6">Sale Entry</h1>
  
      <!-- Individual Entry Form -->
      <div class="bg-white p-6 rounded-lg shadow-md mb-10">
        <form @submit.prevent="handleSubmit" class="space-y-4">
          <div>
            <label for="salesperson" class="block text-sm font-medium text-gray-700 mb-1">Salesperson *</label>
            <select id="salesperson" v-model="form.salespersonId" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option disabled value="">Select Salesperson</option>
              <option v-for="person in salespeople" :key="person.id" :value="person.id">
                {{ person.name }}
              </option>
            </select>
          </div>
  
          <div>
            <label for="date" class="block text-sm font-medium text-gray-700 mb-1">Date *</label>
            <input type="date" id="date" v-model="form.date" required class="block w-full px-4 py-2 border rounded-md shadow-sm" />
          </div>
  
          <div>
            <label for="product" class="block text-sm font-medium text-gray-700 mb-1">Product *</label>
            <select id="product" v-model="form.productId" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option disabled value="">Select Product</option>
              <option v-for="product in products" :key="product.id" :value="product.id">
                {{ product.name }} - {{ product.brand }}
              </option>
            </select>
          </div>
  
          <div>
            <label for="quantity" class="block text-sm font-medium text-gray-700 mb-1">Quantity Sold *</label>
            <input type="number" id="quantity" v-model.number="form.quantity" required min="1" class="block w-full px-4 py-2 border rounded-md shadow-sm" />
          </div>
  
          <div>
            <label for="unitType" class="block text-sm font-medium text-gray-700 mb-1">Unit Type *</label>
            <select id="unitType" v-model="form.unitType" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option value="pieces">Pieces</option>
              <option value="cartons">Cartons</option>
            </select>
          </div>
<!-- Sold Price -->
          <div>
  <label class="block text-sm font-medium text-gray-700 mb-1">Sold Price (per piece) *</label>
  <input
    type="number"
    step="0.01"
    v-model="form.sold_price"
    required
    class="block w-full px-4 py-2 border rounded-md shadow-sm"
  />
</div>

<!-- Customer Name -->
<div>
  <label class="block text-sm font-medium text-gray-700 mb-1">Customer *</label>
  <input
    type="text"
    v-model="form.customer"
    required
    class="block w-full px-4 py-2 border rounded-md shadow-sm"
  />
</div>
  
          <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
            Save Sale Entry
          </button>
        </form>
      </div>
  
      <!-- CSV Upload for Bulk Entry -->
      <div class="bg-white p-6 rounded-lg shadow-md mb-10">
        <h2 class="text-xl font-semibold text-gray-700 mb-4">Bulk Upload Sales (CSV)</h2>
        <input type="file" accept=".csv" @change="handleCSVUpload" class="mb-4" />
      </div>
  
      <!-- Sales Entries List -->
      <div class="bg-white p-6 rounded-lg shadow-md">
        <h2 class="text-xl font-semibold text-gray-700 mb-4">Sales Entries</h2>
        <table v-if="entries.length" class="w-full border border-collapse">
          <thead>
  <tr>
    <th class="border px-3 py-2 text-left">
      <input type="checkbox" @change="toggleSelectAll" :checked="allSelected" />
    </th>
    <th class="border px-3 py-2 text-left">Salesperson</th>
    <th class="border px-3 py-2 text-left">Product</th>
     <th class="border px-3 py-2 text-left">Brand</th>
    <th class="border px-3 py-2 text-left">Date</th>
    <th class="border px-3 py-2 text-left">Quantity</th>
    <th class="border px-3 py-2 text-left">Unit Type</th>
     <th class="border px-3 py-2 text-left">Sold Price</th>
     <th class="border px-3 py-2 text-left">Customer Name</th>
  </tr>
</thead>
        <tbody>
  <tr v-for="entry in entries" :key="entry.id">
    <td class="border px-3 py-2">
      <input type="checkbox" v-model="selectedEntries" :value="entry.id" />
    </td>
    <td class="border px-3 py-2">{{ getSalespersonName(entry.salesperson_id) }}</td>
    <td class="border px-3 py-2">{{ getProductName(entry.product_id) }}</td>
        <td class="border px-3 py-2">{{ getProductBrand(entry.product_id) }}</td>
    <td class="border px-3 py-2">{{ entry.date }}</td>
    <td class="border px-3 py-2">{{ entry.quantity }}</td>
    <td class="border px-3 py-2">{{ entry.unit_type }}</td>
    <td class="border px-3 py-2">{{ entry.sold_price  }}</td>
    <td class="border px-3 py-2">{{ entry.customer }}</td>
  </tr>
</tbody>
        </table>
        <p v-else class="text-gray-500">No sales entries found.</p>
      </div>
      <div v-if="selectedEntries.length > 0" class="mt-4">
  <button
    @click="deleteSelectedEntries"
    class="bg-red-600 text-white px-4 py-2 rounded hover:bg-red-700"
  >
    🗑 Delete Selected ({{ selectedEntries.length }})
  </button>
</div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted, computed,watch  } from 'vue';
  import { apiAddSaleEntry, apiFetchSaleEntries,apiDeleteSaleEntry } from '@/model/sales';
  import { apiFetchSalespeople } from '@/model/api';
  import { apiFetchProducts } from '@/model/products';
  
  const salespeople = ref<any[]>([]);
  const products = ref<any[]>([]);
  const entries = ref<any[]>([]);
  
const form = ref({
  salespersonId: '',
  date: new Date().toISOString().substring(0, 10),
  productId: '',
  quantity: 1,
  unitType: 'pieces',
  brand: '',
  sold_price: 0,
  customer: ''
});
  
 function handleCSVUpload(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = async (e) => {
    const text = e.target?.result as string;
    const lines = text.split('\n').filter(line => line.trim());
    const headers = lines[0].split(',').map(h => h.trim().toLowerCase());

    let successCount = 0;
    let failureCount = 0;

    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(',').map(v => v.trim());
      if (values.length < 5) {
        console.warn(`❌ Row ${i + 1} skipped: Not enough values`);
        failureCount++;
        continue;
      }

      const row: Record<string, string> = {};
      headers.forEach((key, idx) => row[key] = values[idx]);

      const person = salespeople.value.find(p => p.name.toLowerCase() === row.salesperson?.toLowerCase());
      const product = products.value.find(p => p.name.toLowerCase() === row.product?.toLowerCase());

      if (!person) {
        console.warn(`⚠️ Row ${i + 1}: Salesperson '${row.salesperson}' not found.`);
        failureCount++;
        continue;
      }
      if (!product) {
        console.warn(`⚠️ Row ${i + 1}: Product '${row.product}' not found.`);
        failureCount++;
        continue;
      }

      try {
        const payload: NewSaleEntry = {
          salesperson_id: person.id,
          product_id: product.id,
          date: row.date,
          quantity: parseInt(row.quantity),
          unit_type: row.unit_type?.toLowerCase() === 'cartons' ? 'cartons' : 'pieces',
          brand: row.brand || product.brand,
          sold_price: parseFloat(row.sold_price || product.unit_price),
          customer: row.customer || 'Unknown'
        };

        await apiAddSaleEntry(payload);
        successCount++;
      } catch (error) {
        console.error(`❌ Row ${i + 1} failed:`, error);
        failureCount++;
      }
    }

    alert(`✅ Upload complete: ${successCount} rows added, ${failureCount} rows failed.`);
    await loadSalesEntries();
  };

  reader.readAsText(file);
}
  
  async function handleSubmit() {
    try {
   const payload = {
  salesperson_id: parseInt(form.value.salespersonId),
  product_id: parseInt(form.value.productId),
  date: form.value.date,
  quantity: form.value.quantity,
  unit_type: form.value.unitType,
  brand: form.value.brand || null,
  sold_price: form.value.sold_price || null,
  customer: form.value.customer || null
};
      await apiAddSaleEntry(payload);
      alert("✅ Sale entry saved!");
      form.value = {
        salespersonId: '',
        date: new Date().toISOString().substring(0, 10),
        productId: '',
        quantity: 1,
        unitType: 'pieces',
        brand : '',
        sold_price :0.01,
        customer :  '',
      };
      await loadSalesEntries();
    } catch (error) {
      alert("❌ Error saving sale entry.");
      console.error(error);
    }
  }
  
  function getSalespersonName(id: number): string {
    const match = salespeople.value.find(p => p.id === id);
    return match ? match.name : `ID ${id}`;
  }
  
  function getProductName(id: number): string {
    const match = products.value.find(p => p.id === id);
    return match ? `${match.name} - ${match.brand}` : `ID ${id}`;
  }
  
  function getProductBrand(id: number): string {
    const match = products.value.find(p => p.id === id);
    return match ? match.brand : `ID ${id}`;
  }

   function getProductSoldPrice(id: number): string {
    const match = products.value.find(p => p.id === id);
    return match ? match.sold_price : `ID ${id}`;
  }
  async function loadSalesEntries() {
    try {
      entries.value = await apiFetchSaleEntries();
    } catch (error) {
      console.error("Error loading sales entries:", error);
    }
  }

  const selectedEntries = ref<number[]>([]);

const allSelected = computed(() => {
  return entries.value.length > 0 && selectedEntries.value.length === entries.value.length;
});

function toggleSelectAll(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  selectedEntries.value = checked ? entries.value.map(e => e.id) : [];
}

async function deleteSelectedEntries() {
  const confirmed = confirm(`Are you sure you want to delete ${selectedEntries.value.length} entries?`);
  if (!confirmed) return;

  try {
    for (const id of selectedEntries.value) {
      await apiDeleteSaleEntry(id);
    }
    alert("✅ Selected entries deleted.");
    selectedEntries.value = [];
    await loadSalesEntries();
  } catch (error) {
    console.error("❌ Error deleting entries:", error);
    alert("❌ Error deleting one or more entries.");
  }
}

watch(() => form.value.productId, (productId) => {
  const selected = products.value.find(p => p.id === parseInt(productId));
  if (selected) {
    form.value.brand = selected.brand;
    form.value.sold_price = selected.unit_price; // default to product price
  } else {
    form.value.brand = '';
    form.value.sold_price = 0;
  }
});
  
  onMounted(async () => {
    salespeople.value = await apiFetchSalespeople();
    products.value = await apiFetchProducts();
    await loadSalesEntries();
  });
  </script>
  