<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">📈 Campaign Progress</h1>

  
    <table class="reward-summary">
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
           <th class="p-3">Progress</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in progress" :key="`${row.salesperson_id}-${row.product_id}`"   :class="{
    'bg-green-100': row.achieved_quantity >= row.target_quantity,
    'bg-red-100': row.achieved_quantity < row.target_quantity
  }"class="border">
  
          <td class="p-3">{{ row.salesperson_name }}</td>
          <td class="p-3">{{ row.product_name }}</td>
          <td class="p-3">{{ row.target_quantity }}</td>
          <td class="p-3">{{ row.achieved_quantity }}</td>
          <td class="p-3">${{ row.base_reward }}</td>
          <td class="p-3">x{{ row.multiplier }}</td>
          <td class="p-3">{{ row.tier_label || '' }}</td>
          <td class="p-3 font-semibold">${{ row.total_reward.toFixed(2) }}</td>
<td class="p-3 w-48"> <!-- or any fixed width you like -->
  <div class="w-full bg-gray-200 rounded-full h-4 overflow-hidden">
    <div
      class="bg-green-500 h-4"
      :style="{ width: `${Math.min(100, Math.round((row.achieved_quantity / row.target_quantity) * 100))}%` }"
    ></div>
  </div>
  <div class="text-xs text-gray-600 mt-1 text-right">
    {{ Math.round((row.achieved_quantity / row.target_quantity) * 100) }}%
  </div>
</td>
        </tr>
      </tbody>
    </table>

    <h2 class="text-lg font-semibold text-gray-700 mt-8">💰 Incentive Summary</h2>
<table class="incentive-summary-table">
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
import { ref, onMounted, computed }from 'vue';
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
  for (const c of campaigns) {
    if (c.id === campaign_id) {
      campaign.value = c;
      break;
    }
  }






  const allocations = await apiGetTargetAllocations(campaign_id);
  const tiers = await apiGetTargetTiers(campaign_id);
  const sales = await apiFetchSaleEntries();
  const salespeople = await apiFetchSalespeople();
  const campaignProducts = await apiGetProductsForCampaign(campaign_id);

  const rows: any[] = [];

  for (const alloc of allocations) {
    for (const product of campaignProducts) {
      const productSales = [];
      for (const s of sales) {
        if (
          s.salesperson_id === alloc.salesperson_id &&
          s.product_id === product.product_id &&
          s.date >= campaign.value.start_date &&
          s.date <= campaign.value.end_date
        ) {
          productSales.push(s);
        }
      }

      let totalQty = 0;
      for (const s of productSales) {
        totalQty += s.quantity;
      }

      let matchedTier = null;
      for (let i = tiers.length - 1; i >= 0; i--) {
        const tier = tiers[i];
        if (totalQty >= tier.multiplier * alloc.target_quantity) {
          matchedTier = tier;
          break;
        }
      }

      let rewardPerUnit = 0;
      if (totalQty >= alloc.target_quantity) {
        rewardPerUnit = alloc.base_reward;
      }
      if (matchedTier) {
        rewardPerUnit = matchedTier.reward_per_unit;
      }
      const totalReward = rewardPerUnit * totalQty;

      let salespersonName = 'Unknown';
      for (const sp of salespeople) {
        if (sp.id === alloc.salesperson_id) {
          salespersonName = sp.name;
          break;
        }
      }

      rows.push({
        salesperson_id: alloc.salesperson_id,
        salesperson_name: salespersonName,
        product_id: product.product_id,
        product_name: product.product_name,
        target_quantity: alloc.target_quantity,
        achieved_quantity: totalQty,
        base_reward: alloc.base_reward,
        multiplier: matchedTier?.multiplier || 1,
        tier_label: matchedTier?.notes || 'Base',
        total_reward: totalReward
      });
    }
  }

  progress.value = rows;
  console.log("progress",progress.value );
});


</script>

<style scoped>
table {
  background-color :rgb(232, 217, 217);
  border-collapse: collapse;
  width: 100%;
}

.incentive-summary-table {
  background-color :rgb(223, 219, 100);
  border-collapse: collapse;
  width: 100%;
}

reward-summary
th, td {
  text-align: left;
  padding: 8px;
}
th {
  background-color:rgb(236, 236, 236);
}
</style>
