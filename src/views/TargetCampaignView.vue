<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">🎯 Target Campaign Management</h1>

    <div class="text-right mb-4">
      <button
        class="bg-green-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg text-lg shadow-md"
        @click="showForm = !showForm"
      >
        Create New Campaign
      </button>
    </div>

    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">📋 Active Campaigns</h2>
      <div class="overflow-x-auto whitespace-nowrap space-x-4 flex pb-2 px-2">
        <div
          v-for="campaign in activeCampaigns"
          :key="campaign.id"
          class="inline-block bg-gray-100 border rounded-lg shadow-md px-4 py-3 min-w-[280px] mr-2"
        >
          <div class="font-semibold text-gray-800 truncate">
            {{ campaign.name }} ({{ campaign.brand }})
          </div>
          <div class="text-xs text-gray-500">
            {{ campaign.start_date }} → {{ campaign.end_date }}
          </div>
          <ul class="text-xs text-gray-600 mt-1 list-disc list-inside whitespace-normal">
            <li v-for="p in campaign.products" :key="p.product_id">
              {{ p.product_name }}
            </li>
          </ul>
          <button
            class="mt-2 text-sm text-blue-600 underline"
            @click="goToCampaignProgress(campaign.id)"
          >
            View Progress
          </button>
        </div>
      </div>
    </section>

    <CampaignForm v-if="showForm" @refresh="loadActiveCampaigns" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { apiGetCampaigns, apiGetProductsForCampaign, TargetCampaign } from '@/model/incentives';
import CampaignForm from './CampaignForm.vue';

const router = useRouter();
const showForm = ref(false);
const activeCampaigns = ref<TargetCampaign[]>([]);

function goToCampaignProgress(campaign_id: number) {
  router.push({ name: 'CampaignProgress', params: { id: campaign_id } });
}

async function loadActiveCampaigns() {
  const all = await apiGetCampaigns();
  for (const campaign of all) {
    campaign.products = await apiGetProductsForCampaign(campaign.id);
  }
  activeCampaigns.value = all.filter(c => c.is_active);
}

async function archiveCampaign(id: number) {
  if (confirm('Archive this campaign?')) {
    await invoke('archive_campaign', { campaignId: id });
    campaigns.value = campaigns.value.filter(c => c.id !== id);
  }
}

onMounted(loadActiveCampaigns);
</script>

<style scoped>
</style>
