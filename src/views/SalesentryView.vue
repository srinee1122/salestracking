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
              <th class="border px-3 py-2 text-left">Salesperson</th>
              <th class="border px-3 py-2 text-left">Product</th>
              <th class="border px-3 py-2 text-left">Date</th>
              <th class="border px-3 py-2 text-left">Quantity</th>
              <th class="border px-3 py-2 text-left">Unit Type</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="entry in entries" :key="entry.id">
              <td class="border px-3 py-2">{{ getSalespersonName(entry.salesperson_id) }}</td>
              <td class="border px-3 py-2">{{ getProductName(entry.product_id) }}</td>
              <td class="border px-3 py-2">{{ entry.date }}</td>
              <td class="border px-3 py-2">{{ entry.quantity }}</td>
              <td class="border px-3 py-2">{{ entry.unit_type }}</td>
            </tr>
          </tbody>
        </table>
        <p v-else class="text-gray-500">No sales entries found.</p>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { apiAddSaleEntry, apiFetchSaleEntries } from '@/model/sales';
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
  });
  
  function handleCSVUpload(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
  
    const reader = new FileReader();
    reader.onload = async (e) => {
      const text = e.target?.result as string;
      const lines = text.split('\n').filter(line => line.trim());
      const headers = lines[0].split(',').map(h => h.trim());
  
      for (let i = 1; i < lines.length; i++) {
        const values = lines[i].split(',').map(v => v.trim());
        const row: Record<string, string> = {};
        headers.forEach((key, idx) => row[key] = values[idx]);
  
        const person = salespeople.value.find(p => p.name === row.salesperson);
        const product = products.value.find(p => p.name === row.product);
  
        if (person && product) {
          await apiAddSaleEntry({
            salesperson_id: person.id,
            product_id: product.id,
            date: row.date,
            quantity: parseInt(row.quantity),
            unit_type: row.unit_type.toLowerCase() === 'cartons' ? 'cartons' : 'pieces',
          });
        }
      }
  
      alert("✅ Bulk upload complete!");
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
      };
      await apiAddSaleEntry(payload);
      alert("✅ Sale entry saved!");
      form.value = {
        salespersonId: '',
        date: new Date().toISOString().substring(0, 10),
        productId: '',
        quantity: 1,
        unitType: 'pieces',
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
  
  async function loadSalesEntries() {
    try {
      entries.value = await apiFetchSaleEntries();
    } catch (error) {
      console.error("Error loading sales entries:", error);
    }
  }
  
  onMounted(async () => {
    salespeople.value = await apiFetchSalespeople();
    products.value = await apiFetchProducts();
    await loadSalesEntries();
  });
  </script>
  