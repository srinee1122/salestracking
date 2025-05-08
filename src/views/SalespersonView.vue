<template>
    <div class="bg-gray-100 p-4 sm:p-8 font-sans text-gray-800 min-h-screen">
      <div class="container mx-auto max-w-5xl bg-white p-6 sm:p-8 rounded-lg shadow-lg">
        <h1 class="text-3xl font-bold mb-8 text-gray-900">Salesperson Management (Vue)</h1>
  
        <div v-if="message" :class="['p-3 mb-6 rounded-md font-medium text-sm border', message.isError ? 'bg-red-100 text-red-800 border-red-200' : 'bg-green-100 text-green-800 border-green-200']">
          {{ message.text }}
        </div>
  
        <div class="mb-10 p-6 border border-gray-200 rounded-lg bg-gray-50">
          <h2 class="text-xl font-semibold mb-5 text-gray-700">Add New Salesperson</h2>
          <form @submit.prevent="handleAddSalesperson" class="space-y-5">
            <div>
              <label for="name" class="block text-sm font-medium text-gray-700 mb-1">Name:</label>
              <input
                type="text"
                id="name"
                v-model="formState.name"
                required
                class="block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm bg-white focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="employee-id" class="block text-sm font-medium text-gray-700 mb-1">Employee ID (Optional):</label>
              <input
                type="text"
                id="employee-id"
                v-model="formState.employeeId"
                class="block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm bg-white focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="email" class="block text-sm font-medium text-gray-700 mb-1">Email (Optional):</label>
              <input
                type="email"
                id="email"
                v-model="formState.email"
                class="block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm bg-white focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              />
            </div>
            <button
              type="submit"
              class="inline-flex justify-center py-2 px-5 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out disabled:opacity-50"
              :disabled="isLoading" >
              {{ isLoading ? 'Adding...' : 'Add Salesperson' }}
            </button>
          </form>
        </div>
  
        <div class="p-6 border border-gray-200 rounded-lg">
          <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-5">
            <h2 class="text-xl font-semibold text-gray-700 mb-3 sm:mb-0">Salespeople List</h2>
            <button
              @click="loadSalespeople"
              class="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out disabled:opacity-50"
              :disabled="isLoading" >
               {{ isLoading ? 'Loading...' : 'Refresh List' }}
            </button>
          </div>
  
          <div class="overflow-x-auto shadow border-b border-gray-200 rounded-lg">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-gray-100">
                <tr>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-bold text-gray-600 uppercase tracking-wider">ID</th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-bold text-gray-600 uppercase tracking-wider">Name</th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-bold text-gray-600 uppercase tracking-wider">Employee ID</th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-bold text-gray-600 uppercase tracking-wider">Email</th>
                  <th scope="col" class="px-6 py-3 text-left text-xs font-bold text-gray-600 uppercase tracking-wider">Active</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr v-if="isLoading && salespeople.length === 0">
                  <td colspan="5" class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-center italic">Loading...</td>
                </tr>
                <tr v-else-if="!isLoading && salespeople.length === 0">
                  <td colspan="5" class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-center">No salespeople found.</td>
                </tr>
                <tr v-for="person in salespeople" :key="person.id">
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{{ person.id }}</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-700">{{ person.name }}</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ person.employee_id ?? 'N/A' }}</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ person.email ?? 'N/A' }}</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm">
                    <span :class="['px-2 inline-flex text-xs leading-5 font-semibold rounded-full', person.is_active ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800']">
                      {{ person.is_active ? 'Active' : 'Inactive' }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
  
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  // Using <script setup> enables Composition API syntax directly
  import { ref, reactive, onMounted } from 'vue';
  // Import the API functions and types from our model file
  // Make sure the path './model/api' is correct relative to App.vue
  import { apiFetchSalespeople, apiAddSalesperson, Salesperson, AddSalespersonPayload } from '../model/api';
 
  
  // --- Reactive State Definition ---
  
  // Holds the array of salespeople fetched from the backend
  const salespeople = ref<Salesperson[]>([]);
  
  // Holds the data for the 'Add New Salesperson' form
  const formState = reactive<AddSalespersonPayload>({
    name: '',
    employeeId: null, // Initialize optional fields to null
    email: null,
  });

  
  
  // Tracks whether an API call is in progress
  const isLoading = ref<boolean>(false);
  
  // Holds the current status message
  const message = ref<{ text: string; isError: boolean } | null>(null);
  
  // --- Methods ---
  
  /**
   * Displays a status message and automatically clears it after a delay.
   */
  function showMessage(text: string, isError: boolean = false): void {
    message.value = { text, isError };
    setTimeout(() => {
      message.value = null;
    }, 5000);
  }
  
  /**
   * Fetches the list of salespeople from the backend API.
   */
  async function loadSalespeople(): Promise<void> {
    console.log("Vue: Loading salespeople...");
    isLoading.value = true;
    message.value = null;
    try {
      salespeople.value = await apiFetchSalespeople();
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      showMessage(`Error loading salespeople: ${errorMessage}`, true);
      salespeople.value = [];
    } finally {
      isLoading.value = false;
    }
  }
  
  /**
   * Handles the submission of the 'Add New Salesperson' form.
   */
  async function handleAddSalesperson(): Promise<void> {
    console.log("Vue: Handling add salesperson...");
    if (!formState.name) {
        showMessage("Name is required.", true);
        return;
    }
    isLoading.value = true;
    message.value = null;
  
    const payload: AddSalespersonPayload = {
        name: formState.name.trim(),
        employeeId: formState.employeeId?.trim() || null,
        email: formState.email?.trim() || null,
    };
  
    try {
      await apiAddSalesperson(payload);
      showMessage("Salesperson added successfully!", false);
      // Reset form fields
      formState.name = '';
      formState.employeeId = null;
      formState.email = null;
      await loadSalespeople(); // Refresh the list
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      showMessage(`Error adding salesperson: ${errorMessage}`, true);
    } finally {
      isLoading.value = false;
    }
  }
  
  // --- Lifecycle Hook ---
  
  /**
   * Runs after the component is mounted to the DOM.
   * Fetches the initial list of salespeople.
   */
  onMounted(() => {
    console.log("Vue: Component mounted, loading initial data.");
    loadSalespeople();
  });
  
  </script>
  
  <style>
  /* Scoped styles or global styles can be added here */
  /* Most styling is handled by Tailwind classes in the template */
  
  /* Example: Add slight transition to message appearance */
  .message {
    transition: opacity 0.3s ease;
  }
  </style>
  