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
          <th class="p-3">Progress</th>
          <th class="p-3">Reward</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="row in progress"
          :key="`${row.salesperson_id}-${row.product_id}`"
          class="border-t"
        >
          <td class="p-3">{{ row.salesperson_name }}</td>
          <td class="p-3">{{ row.product_name }}</td>
          <td class="p-3">{{ row.target_quantity }} ({{ row.target_unit }})</td>
          <td class="p-3">{{ row.achieved_quantity }}</td>
          <td class="p-3 w-1/3">
            <div class="w-full bg-gray-200 rounded-full h-4">
              <div
                class="h-4 bg-green-500 text-xs text-white text-center rounded-full transition-all duration-500"
                :class="{ 'animate-pulse': row.multiplier > 1 }"
                :style="{ width: `${Math.min(row.progress_percent, 100)}%` }"
              >
                {{ row.progress_percent }}%
              </div>
            </div>
          </td>
          <td class="p-3 font-semibold">${{ row.total_reward.toFixed(2) }}</td>
        </tr>
      </tbody>
    </table>

    <h2 class="text-lg font-semibold text-gray-700 mt-8">💰 Incentive Summary</h2>
    <table class="min-w-full bg-white border rounded shadow mt-2">
      <thead>
        <tr class="bg-gray-100 text-left text-sm font-semibold text-gray-700">
          <th class="p-3">Salesperson</th>
          <th class="p-3">Total Incentive Earned</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(amount, name) in incentiveSummary" :key="name" class="border-t">
          <td class="p-3">{{ name }}</td>
          <td class="p-3 font-semibold">${{ amount.toFixed(2) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import {
  apiGetCampaigns,
  apiGetTargetAllocations,
  apiGetTargetTiers,
  apiGetProductsForCampaign,
} from '@/model/incentives';
import { apiFetchSalespeople } from '@/model/api';
import { apiFetchSaleEntries } from '@/model/sales';
import { apiFetchProducts } from '@/model/products';

const route = useRoute();
const campaign_id = parseInt(route.params.id as string);
const campaign = ref<any>(null);
const progress = ref<any[]>([]);

const incentiveSummary = computed(() => {
  const summary: Record<string, number> = {};
  for (const row of progress.value) {
    if (!summary[row.salesperson_name]) {
      summary[row.salesperson_name] = 0;
    }
    summary[row.salesperson_name] += row.total_reward;
  }
  return summary;
});

onMounted(async () => {
  const campaigns = await apiGetCampaigns();
  campaign.value = campaigns.find((c) => c.id === campaign_id);

  const allocations = await apiGetTargetAllocations(campaign_id);
  const tiers = await apiGetTargetTiers(campaign_id);
  const sales = await apiFetchSaleEntries();
  const salespeople = await apiFetchSalespeople();
  const campaignProducts = await apiGetProductsForCampaign(campaign_id);
  const allProducts = await apiFetchProducts();

  const rows: any[] = [];

  for (const alloc of allocations) {
    for (const product of campaignProducts) {
      const productMeta = allProducts.find((p) => p.id === product.product_id);
      const cartonSize = productMeta?.carton_size || 1;

      const productSales = sales.filter(
        (s) =>
          s.salesperson_id === alloc.salesperson_id &&
          s.product_id === product.product_id &&
          s.date >= campaign.value.start_date &&
          s.date <= campaign.value.end_date
      );

      const totalQty = productSales.reduce((sum, s) => {
        if (alloc.target_unit === 'cartons' && s.unit_type === 'pieces') {
          return sum + s.quantity / cartonSize;
        }
        return sum + s.quantity;
      }, 0);

      const matchedTier = [...tiers].reverse().find((tier) => {
        const threshold = tier.multiplier * alloc.target_quantity;
        return totalQty >= threshold;
      });

      let rewardPerUnit = matchedTier?.reward_per_unit || (totalQty >= alloc.target_quantity ? alloc.base_reward : 0);
      const totalReward = rewardPerUnit * totalQty;
      const percent = Math.round((totalQty / alloc.target_quantity) * 100);

      rows.push({
        salesperson_id: alloc.salesperson_id,
        salesperson_name: salespeople.find((sp) => sp.id === alloc.salesperson_id)?.name || 'Unknown',
        product_id: product.product_id,
        product_name: product.product_name,
        target_quantity: alloc.target_quantity,
        achieved_quantity: totalQty,
        base_reward: alloc.base_reward,
        multiplier: matchedTier?.multiplier || 1,
        tier_label: matchedTier?.notes || 'Base',
        total_reward: totalReward,
        progress_percent: percent,
        target_unit: alloc.target_unit || 'pieces',
      });
    }
  }

  progress.value = rows;
});
</script>

<style scoped>
.animate-pulse {
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0% { box-shadow: 0 0 5px #22c55e; }
  50% { box-shadow: 0 0 20px rgb(197, 194, 34); }
  100% { box-shadow: 0 0 5px #22c55e; }
}
</style>
