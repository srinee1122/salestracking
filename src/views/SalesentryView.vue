<template>
    <div class="p-6 sm:p-8">
      <h1 class="text-2xl font-bold text-gray-800 mb-6">Sale Entry</h1>
  
      <div class="bg-white p-6 rounded-lg shadow-md">
        <form @submit.prevent="handleSubmit" class="space-y-4">
          <!-- Salesperson -->
          <div>
            <label for="salesperson" class="block text-sm font-medium text-gray-700 mb-1">Salesperson *</label>
            <select id="salesperson" v-model="form.salespersonId" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option disabled value="">Select Salesperson</option>
              <option v-for="person in salespeople" :key="person.id" :value="person.id">
                {{ person.name }}
              </option>
            </select>
          </div>
  
          <!-- Date -->
          <div>
            <label for="date" class="block text-sm font-medium text-gray-700 mb-1">Date *</label>
            <input type="date" id="date" v-model="form.date" required class="block w-full px-4 py-2 border rounded-md shadow-sm" />
          </div>
  
          <!-- Product -->
          <div>
            <label for="product" class="block text-sm font-medium text-gray-700 mb-1">Product *</label>
            <select id="product" v-model="form.productId" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option disabled value="">Select Product</option>
              <option v-for="product in products" :key="product.id" :value="product.id">
                {{ product.name }} - {{ product.brand }}
              </option>
            </select>
          </div>
  
          <!-- Quantity -->
          <div>
            <label for="quantity" class="block text-sm font-medium text-gray-700 mb-1">Quantity Sold *</label>
            <input type="number" id="quantity" v-model.number="form.quantity" required min="1" class="block w-full px-4 py-2 border rounded-md shadow-sm" />
          </div>
  
          <!-- Unit Type -->
          <div>
            <label for="unitType" class="block text-sm font-medium text-gray-700 mb-1">Unit Type *</label>
            <select id="unitType" v-model="form.unitType" required class="block w-full px-4 py-2 border rounded-md shadow-sm">
              <option value="pieces">Pieces</option>
              <option value="cartons">Cartons</option>
            </select>
          </div>
  
          <!-- Submit -->
          <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
            Save Sale Entry
          </button>
        </form>

        <div class="mt-10">
  <h2 class="text-lg font-semibold mb-4">Sales Entries</h2>
  <table class="min-w-full bg-white border border-gray-300 text-sm">
    <thead class="bg-gray-100">
      <tr>
        <th class="px-3 py-2 text-left">Date</th>
        <th class="px-3 py-2 text-left">Salesperson</th>
        <th class="px-3 py-2 text-left">Product</th>
        <th class="px-3 py-2 text-left">Qty</th>
        <th class="px-3 py-2 text-left">Unit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="entry in salesEntries" :key="entry.id" class="border-t">
        <td>{{ entry.date }}</td>
  <td>{{ getSalespersonName(entry.salesperson_id) }}</td>
  <td>{{ getProductName(entry.product_id) }}</td>
  <td>{{ entry.quantity }}</td>
  <td>{{ entry.unit_type }}</td>
      </tr>
    </tbody>
  </table>
</div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { apiFetchSalespeople } from '@/model/api';
  import { apiFetchProducts } from '@/model/products';
   import { apiAddSaleEntry, apiFetchSaleEntries, SaleEntry } from '@/model/sales';

  const salespeople = ref<any[]>([]);
  const products = ref<any[]>([]);
 const salesEntries = ref<SaleEntry[]>([]);

  const form = ref({
    salespersonId: '',
    date: new Date().toISOString().substring(0, 10),
    productId: '',
    quantity: 1,
    unitType: 'pieces',
  });
  
 

async function loadSalesEntries() {
  try {
    salesEntries.value = await apiFetchSaleEntries();
  } catch (error) {
    alert("Failed to load sales entries.");
    console.error(error);
  }
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

    await loadSalesEntries(); // 👈 Fetch the updated list

    // Reset the form
    form.value = {
      salespersonId: '',
      date: new Date().toISOString().substring(0, 10),
      productId: '',
      quantity: 1,
      unitType: 'pieces',
    };
  } catch (error) {
    alert("❌ Error saving sale entry.");
    console.error(error);
  }
}
  
onMounted(async () => {
  try {
    salespeople.value = await apiFetchSalespeople();
    products.value = await apiFetchProducts();
    salesEntries.value = await apiFetchSaleEntries();
  } catch (error) {
    alert("Failed to load sales entries.");
    console.error(error);
  }
});

function getSalespersonName(id: number): string {
  const person = salespeople.value.find(p => p.id === id);
  return person ? person.name : 'Unknown';
}

function getProductName(id: number): string {
  const product = products.value.find(p => p.id === id);
  return product ? `${product.name} - ${product.brand}` : 'Unknown';
}
  </script>
  