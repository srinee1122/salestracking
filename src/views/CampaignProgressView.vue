<template>
    <div class="p-6 sm:p-8">
      <h1 class="text-2xl font-bold text-gray-800 mb-6">📊 Campaign Progress</h1>
  
      <div v-if="campaign" class="mb-6">
        <h2 class="text-xl font-semibold text-gray-700">{{ campaign.name }} ({{ campaign.brand }})</h2>
        <p class="text-gray-600">{{ campaign.start_date }} to {{ campaign.end_date }}</p>
      </div>
  
      <div v-if="progress.length" class="bg-white rounded-lg shadow p-4">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Salesperson</th>
              <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Target Qty</th>
              <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Achieved Qty</th>
              <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="row in progress" :key="row.salesperson">
              <td class="px-4 py-2 text-sm text-gray-700">{{ row.salesperson }}</td>
              <td class="px-4 py-2 text-sm text-gray-700">{{ row.target }}</td>
              <td class="px-4 py-2 text-sm text-gray-700">{{ row.achieved }}</td>
              <td class="px-4 py-2 text-sm">
                <span :class="row.achieved >= row.target ? 'text-green-600 font-semibold' : 'text-red-500 font-medium'">
                  {{ row.achieved >= row.target ? 'Met' : 'Pending' }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <p v-else class="text-gray-500">No progress data available.</p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRoute } from 'vue-router';
  import { apiGetCampaigns } from '@/model/incentives';
  
  const route = useRoute();
  const campaignId = parseInt(route.params.id as string);
  
  const campaign = ref<any>(null);
  const progress = ref<any[]>([]); // Will hold rows: { salesperson, target, achieved }
  
  onMounted(async () => {
    const allCampaigns = await apiGetCampaigns();
    campaign.value = allCampaigns.find(c => c.id === campaignId);
  
    // This would eventually come from a real API call (placeholder for now)
    progress.value = [
      { salesperson: 'Arun', target: 100, achieved: 120 },
      { salesperson: 'Manoj', target: 75, achieved: 65 },
      { salesperson: 'Vaithi', target: 50, achieved: 50 }
    ];
  });
  </script>
  