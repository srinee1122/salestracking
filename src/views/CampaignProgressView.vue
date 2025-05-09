<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">📈 Campaign Progress</h1>

    <div v-if="campaign" class="mb-6">
      <h2 class="text-lg font-semibold text-gray-700">{{ campaign.name }} ({{ campaign.brand }})</h2>
      <p class="text-sm text-gray-600">{{ campaign.start_date }} to {{ campaign.end_date }}</p>
    </div>

    <table class="min-w-full bg-white border rounded shadow">
      <thead>
        <tr class="bg-gray-100 text-left text-sm font-semibold text-gray-700">
          <th class="p-3">Salesperson</th>
          <th class="p-3">Product</th>
          <th class="p-3">Target</th>
          <th class="p-3">Achieved</th>
          <th class="p-3">Base Reward</th>
          <th class="p-3">Tier</th>
          <th class="p-3">Tier Label</th>
          <th class="p-3">Total Reward</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in progress" :key="`${row.salesperson_id}-${row.product_id}`" class="border-t">
          <td class="p-3">{{ row.salesperson_name }}</td>
          <td class="p-3">{{ row.product_name }}</td>
          <td class="p-3">{{ row.target_quantity }}</td>
          <td class="p-3">{{ row.achieved_quantity }}</td>
          <td class="p-3">${{ row.base_reward }}</td>
          <td class="p-3">x{{ row.multiplier }}</td>
          <td class="p-3">{{ row.tier_label || '—' }}</td>
          <td class="p-3 font-semibold">${{ row.total_reward.toFixed(2) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import {
  apiGetCampaigns,
  apiGetTargetAllocations,
  apiGetTargetTiers,
  apiGetProductsForCampaign
} from '@/model/incentives';
import { apiFetchSalespeople } from '@/model/api';
import { apiFetchSaleEntries } from '@/model/sales';

const route = useRoute();
const campaign_id = parseInt(route.params.id as string);
const campaign = ref<any>(null);
const progress = ref<any[]>([]);

onMounted(async () => {
  const campaigns = await apiGetCampaigns();
  campaign.value = campaigns.find((c) => c.id === campaign_id);
  const allocations = await apiGetTargetAllocations(campaign_id);
  const tiers = await apiGetTargetTiers(campaign_id);
  const sales = await apiFetchSaleEntries();
  const salespeople = await apiFetchSalespeople();
  const campaignProducts = await apiGetProductsForCampaign(campaign_id);

  const rows: any[] = [];

  for (const alloc of allocations) {
    for (const product of campaignProducts) {
      const productSales = sales.filter(
        (s) =>
          s.salesperson_id === alloc.salesperson_id &&
          s.product_id === product.product_id &&
          s.date >= campaign.value.start_date &&
          s.date <= campaign.value.end_date
      );

      const totalQty = productSales.reduce((sum, s) => sum + s.quantity, 0);
      const achievedRatio = totalQty / alloc.target_quantity;

      // 🚫 Skip if salesperson did not meet the target
      if (achievedRatio < 1) continue;

      // ✅ Find highest tier met by achieved ratio
      const tier = [...tiers]
        .sort((a, b) => b.multiplier - a.multiplier)
        .find((t) => achievedRatio >= t.multiplier) || {
          multiplier: 1,
          reward_per_unit: alloc.base_reward,
          notes: '',
        };

      const rewardPerUnit = tier.reward_per_unit;
      const totalReward = totalQty * rewardPerUnit;

      rows.push({
        salesperson_id: alloc.salesperson_id,
        salesperson_name: salespeople.find((sp) => sp.id === alloc.salesperson_id)?.name || 'Unknown',
        product_id: product.product_id,
        product_name: product.name,
        target_quantity: alloc.target_quantity,
        achieved_quantity: totalQty,
        base_reward: alloc.base_reward,
        multiplier: tier.multiplier,
        tier_label: tier.notes,
        total_reward: totalReward
      });
    }
  }

  progress.value = rows;
});
</script>

<style scoped>
table {
  border-collapse: collapse;
  width: 100%;
}
th, td {
  text-align: left;
  padding: 8px;
}
th {
  background-color: #f4f4f4;
}
</style>
