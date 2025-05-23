<template>
  <div class="bg-white p-6 rounded-lg shadow mb-8 border border-blue-300">
    <h2 class="text-xl font-semibold text-gray-700 mb-4">🆕 Create New Campaign</h2>

    <!-- Campaign Info -->
    <section class="mb-6">
      <h3 class="text-lg font-medium text-gray-800 mb-2">1. Campaign Info</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <input v-model="campaignForm.name" placeholder="Campaign Name" class="input" />
        <select v-model="campaignForm.brand" class="input">
          <option disabled value="">Select Brand</option>
          <option v-for="brand in uniqueBrands" :key="brand" :value="brand">{{ brand }}</option>
        </select>
        <input type="date" v-model="campaignForm.start_date" class="input" />
        <input type="date" v-model="campaignForm.end_date" class="input" />
      </div>
    </section>

    <!-- Product Selection -->
    <section class="mb-6">
      <h3 class="text-lg font-medium text-gray-800 mb-2">2. Choose Products</h3>
      <select v-model="selectedProducts" multiple class="w-full h-40 border p-2 rounded">
        <option v-for="product in filteredProducts" :key="product.id" :value="product.id">
          {{ product.name }} - {{ product.brand }}
        </option>
      </select>
    </section>

    <!-- Sales Targets -->
    <section class="mb-6">
      <h3 class="text-lg font-medium text-gray-800 mb-2">3. Assign Sales Targets</h3>
      <div v-for="person in salespeople" :key="person.id" class="grid grid-cols-1 sm:grid-cols-4 gap-2 mb-2">
        <label class="font-medium">{{ person.name }}</label>
        <input type="number" v-model.number="salesTargets[person.id]" placeholder="Min Qty" class="input" />
        <input type="number" v-model.number="baseRewards[person.id]" placeholder="Base Reward/Unit" class="input" />
        <select v-model="targetunits[person.id]" class="input">
          <option disabled value="">Unit</option>
          <option value="pieces">Pieces</option>
          <option value="cartons">Cartons</option>
        </select>
      </div>
    </section>

   <!-- Tiers -->
<section class="mb-6">
  <h3 class="text-lg font-medium text-gray-800 mb-4">4. Incentive Tiers</h3>

  <!-- Table Header for New Entry -->
  <div class="grid grid-cols-4 gap-2 font-semibold text-gray-700 mb-2 text-sm">
    <div>Multiplier (x)</div>
    <div>Reward/Unit ($)</div>
    <div>Tier Label</div>
    <div></div> <!-- for the add button -->
  </div>

  <!-- Tier Input Row -->
  <form @submit.prevent="addTier" class="grid grid-cols-4 gap-2 mb-4">
    <input
      type="number"
      step="0.1"
      min="0.1"
      v-model.number="tierForm.multiplier"
      placeholder="e.g. 1.2"
      class="input"
    />
    <input
      type="number"
      step="0.01"
      min="0"
      v-model.number="tierForm.reward_per_unit"
      placeholder="e.g. 0.75"
      class="input"
    />
    <input
      type="text"
      v-model="tierForm.notes"
      placeholder="e.g. Silver Tier"
      class="input"
    />
    <button type="submit" class="btn-outline">➕ Add</button>
  </form>

 <!-- Existing Tier Table -->
  <div v-if="tierList.length" class="mt-4 border rounded overflow-hidden">
    <div class="grid grid-cols-3 bg-gray-100 text-gray-700 font-semibold text-sm p-2 border-b">
      <div>Multiplier (x)</div>
      <div>Reward/Unit ($)</div>
      <div>Tier Label</div>
    </div>
    <div
      v-for="tier in tierList"
      :key="`${tier.multiplier}-${tier.reward_per_unit}`"
      class="grid grid-cols-3 p-2 border-b text-sm text-gray-800"
    >
      <div>x{{ tier.multiplier }}</div>
      <div>${{ tier.reward_per_unit.toFixed(2) }}</div>
      <div>{{ tier.notes || 'Base' }}</div>
    </div>
  </div>
</section>


    <!-- Save Button -->
    <div class="text-right">
      <button class="btn-blue" @click="saveFullCampaign">📂 Save Campaign</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, defineEmits } from 'vue';
import {
  apiCreateCampaign,
  apiSetCampaignProducts,
  apiAddTargetAllocation,
  apiAddTargetTier,
  apiGetCampaigns
} from '@/model/incentives';
import { apiFetchProducts } from '@/model/products';
import { apiFetchSalespeople } from '@/model/api';

const emit = defineEmits(['refresh']);

const campaignForm = ref({ name: '', brand: '', start_date: '', end_date: '' });
const products = ref<any[]>([]);
const salespeople = ref<any[]>([]);
const selectedProducts = ref<number[]>([]);
const salesTargets = ref<Record<number, number>>({});
const baseRewards = ref<Record<number, number>>({});
const targetunits = ref<Record<number, string>>({});
const tierForm = ref({ multiplier: 1, reward_per_unit: 0, notes: '' });
const tierList = ref<any[]>([]);

const uniqueBrands = computed(() => {
  const seen = new Set();
  return products.value.map(p => p.brand).filter(brand => {
    if (!seen.has(brand)) {
      seen.add(brand);
      return true;
    }
    return false;
  });
});

const filteredProducts = computed(() => {
  return products.value.filter(p => p.brand === campaignForm.value.brand);
});

function addTier() {
  if (tierList.value.length >= 3) {
    alert('⚠️ You can only add up to 3 tiers.');
    return;
  }
  tierList.value.push({
    multiplier: tierForm.value.multiplier,
    min_quantity: tierForm.value.multiplier,
    reward_per_unit: tierForm.value.reward_per_unit,
    notes: tierForm.value.notes
  });
  tierForm.value = { multiplier: 1, reward_per_unit: 0, notes: '' };
}

async function saveFullCampaign() {
  try {
    await apiCreateCampaign(campaignForm.value);
    const allCampaigns = await apiGetCampaigns();
    const newCampaign = allCampaigns[allCampaigns.length - 1];
    await apiSetCampaignProducts(newCampaign.id, selectedProducts.value);

    for (const person of salespeople.value) {
      const qty = salesTargets.value[person.id];
      const reward = baseRewards.value[person.id];
      const unit = targetunits.value[person.id];

      if (qty > 0 && reward > 0 && unit) {
        for (const product_id of selectedProducts.value) {
          await apiAddTargetAllocation({
            campaign_id: newCampaign.id,
            salesperson_id: person.id,
            product_id,
            target_quantity: qty,
            base_reward: reward,
            target_unit: unit
          });
        }
      }
    }

    for (const tier of tierList.value) {
      await apiAddTargetTier({
        campaign_id: newCampaign.id,
        multiplier: tier.multiplier,
        min_quantity: tier.min_quantity,
        reward_per_unit: tier.reward_per_unit,
        notes: tier.notes
      });
    }

    alert('✅ Campaign created successfully!');
    emit('refresh');
  } catch (err) {
    console.error(err);
    alert('❌ Failed to create campaign.');
  }
}

onMounted(async () => {
  products.value = await apiFetchProducts();
  salespeople.value = await apiFetchSalespeople();
});
</script>

<style scoped>
.input {
  @apply border border-gray-300 text-center rounded px-3 py-2 w-full;
}

.btn-blue {
  @apply bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded shadow;
}

.btn-outline {
  @apply border border-blue-500 text-blue-500 px-4 py-2 rounded hover:bg-blue-50;
}
</style>
