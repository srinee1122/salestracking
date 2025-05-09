<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">🎯 Target Campaign Management</h1>

    <!-- Active Campaigns Section -->
    <section v-if="activeCampaigns.length" class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">📋 Active Campaigns</h2>
      <ul class="list-disc list-inside text-sm text-gray-700">
<li v-for="campaign in activeCampaigns" :key="campaign.id">
  {{ campaign.name }} ({{ campaign.brand }}) — {{ campaign.start_date }} to {{ campaign.end_date }}
  <ul class="ml-4 text-xs text-gray-600">
    <li v-for="p in campaign.products" :key="p.product_id">
      • {{ p.product_name }} ({{ p.brand }})
    </li>
  </ul>
  <button class="btn-outline ml-4" @click="goToCampaignProgress(campaign.id)">View Progress</button>
</li>
      </ul>
    </section>

    <!-- Step 1: Campaign Info -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">1. Campaign Info</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <input v-model="campaignForm.name" placeholder="Campaign Name" class="input" required />
        <select v-model="campaignForm.brand" class="input" required>
          <option disabled value="">Select Brand</option>
          <option v-for="brand in uniqueBrands" :key="brand" :value="brand">{{ brand }}</option>
        </select>
        <input type="date" v-model="campaignForm.start_date" class="input" required />
        <input type="date" v-model="campaignForm.end_date" class="input" required />
      </div>
    </section>

    <!-- Step 2: Choose Products -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">2. Choose Products</h2>
      <select v-model="selectedProducts" multiple class="w-full h-40 border p-2 rounded">
        <option v-for="product in filteredProducts" :value="product.id" :key="product.id">
          {{ product.name }} - {{ product.brand }}
        </option>
      </select>

      <div v-if="assignedProductList.length" class="mt-4">
        <h3 class="text-sm font-semibold text-gray-600 mb-1">Assigned Products:</h3>
        <ul class="list-disc list-inside text-sm text-gray-700">
          <li v-for="p in assignedProductList" :key="p.id">{{ p.name }} - {{ p.brand }}</li>
        </ul>
      </div>
    </section>

    <!-- Step 3: Assign Targets with Base Incentive -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">3. Assign Sales Targets</h2>
      <div v-for="person in salespeople" :key="person.id" class="mb-4 grid grid-cols-1 sm:grid-cols-3 gap-4">
        <label class="font-medium">{{ person.name }}</label>
        <input type="number" v-model.number="salesTargets[person.id]" placeholder="Min Qty" class="input" />
        <input type="number" v-model.number="baseRewards[person.id]" placeholder="Base Reward/Unit" class="input" />
      </div>
    </section>

    <!-- Step 4: Define Tier Multipliers -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">4. Incentive Tiers (Multipliers)</h2>
      <form @submit.prevent="addTier" class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-4">
        <input type="number" step="0.1" v-model.number="tierForm.multiplier" placeholder="Multiplier (e.g., 1.5)" class="input" required />
        <input type="number" v-model.number="tierForm.reward_per_unit" placeholder="Reward/Unit" class="input" required />
        <input type="text" v-model="tierForm.notes" placeholder="Tier Label (optional)" class="input" />
        <button type="submit" class="btn-outline sm:col-span-3">➕ Add Incentive Tier</button>
      </form>
      <ul class="list-disc list-inside text-sm text-gray-700">
        <li v-for="tier in tierList" :key="tier.multiplier">
          x{{ tier.multiplier }} - Reward: ${{ tier.reward_per_unit }} ({{ tier.notes || 'No label' }})
        </li>
      </ul>
    </section>

    <!-- Step 5: Summary -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">5. Summary Before Saving</h2>
      <ul class="text-sm text-gray-700 list-disc list-inside">
        <li><strong>Campaign:</strong> {{ campaignForm.name }} ({{ campaignForm.brand }})</li>
        <li><strong>Dates:</strong> {{ campaignForm.start_date }} to {{ campaignForm.end_date }}</li>
        <li><strong>Products:</strong>
          <ul class="list-disc ml-6">
            <li v-for="p in assignedProductList" :key="p.id">{{ p.name }}</li>
          </ul>
        </li>
        <li><strong>Sales Targets & Base Rewards:</strong>
          <ul class="list-disc ml-6">
            <li v-for="person in salespeople" :key="person.id">
              {{ person.name }} - {{ salesTargets[person.id] || 0 }} units, ${{ baseRewards[person.id] || 0 }}/unit
            </li>
          </ul>
        </li>
        <li><strong>Multipliers:</strong>
          <ul class="list-disc ml-6">
            <li v-for="tier in tierList" :key="tier.multiplier">
              x{{ tier.multiplier }} - Reward: ${{ tier.reward_per_unit }} ({{ tier.notes || 'No label' }})
            </li>
          </ul>
        </li>
      </ul>
    </section>

    <!-- Final Step: Save Campaign -->
    <div class="text-right mt-6">
      <button class="btn-blue" @click="saveFullCampaign">📂 Save Full Campaign</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import {
  apiCreateCampaign,
  apiSetCampaignProducts,
  apiAddTargetAllocation,
  apiAddTargetTier,
  apiGetCampaigns,
  TargetCampaign,
  apiGetProductsForCampaign 
} from '@/model/incentives';
import { apiFetchProducts } from '@/model/products';
import { apiFetchSalespeople } from '@/model/api';
import { useRouter } from 'vue-router';
const router = useRouter();



const campaignForm = ref({ name: '', brand: '', start_date: '', end_date: '' });
const products = ref<any[]>([]);
const salespeople = ref<any[]>([]);
const selectedProducts = ref<number[]>([]);
const salesTargets = ref<Record<number, number>>({});
const baseRewards = ref<Record<number, number>>({});
const tierForm = ref({ multiplier: 1, reward_per_unit: 1, notes: '' });
const tierList = ref<any[]>([]);
const activeCampaigns = ref<TargetCampaign[]>([]);

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

const assignedProductList = computed(() => {
  return products.value.filter(p => selectedProducts.value.includes(p.id));
});

function goToCampaignProgress(campaign_id: number) {
  console.log("Navigating to campaign progress for ID:", campaign_id);
  router.push({ name: 'CampaignProgress', params: { id: campaign_id } });
}


function addTier() {
  const safeMultiplier = parseFloat(tierForm.value.multiplier?.toString() || '1');
  tierList.value.push({
    multiplier: safeMultiplier,
    min_quantity: Math.round(safeMultiplier), // ✅ make sure it’s always set
    reward_per_unit: tierForm.value.reward_per_unit || 0,
    notes: tierForm.value.notes || ''
  });
  tierForm.value = { multiplier: 1, reward_per_unit: 0, notes: '' };
}

async function saveFullCampaign() {
  try {
    await apiCreateCampaign(campaignForm.value);
    const latestCampaigns = await apiGetCampaigns();
    const newCampaign = latestCampaigns[latestCampaigns.length - 1];
    if (!newCampaign) return;

    // Assign selected products to campaign
    await apiSetCampaignProducts(newCampaign.id, selectedProducts.value);

    // Save target allocations (with base reward)
    for (const person of salespeople.value) {
      const qty = salesTargets.value[person.id] || 0;
      const reward = baseRewards.value[person.id] || 0;
      if (qty > 0 && reward > 0) {
        await apiAddTargetAllocation({
          campaign_id: newCampaign.id,
          salesperson_id: person.id,
          target_quantity: qty,
          base_reward: reward
        });
      }
    }

    // Save incentive tiers
for (const tier of tierList.value) {
  await apiAddTargetTier({
    campaign_id: newCampaign.id,
    multiplier: tier.multiplier ?? 1, // <- enforce it
    min_quantity: tier.min_quantity ?? Math.round(tier.multiplier ?? 1),
    reward_per_unit: tier.reward_per_unit ?? 0,
    notes: tier.notes || ''
  });
}

    alert('✅ Full campaign created successfully!');
    await loadActiveCampaigns();
  } catch (err) {
    console.error(err);
    alert('❌ Failed to save campaign.');
  }
}

async function loadActiveCampaigns() {
  const all = await apiGetCampaigns();
  for (const campaign of all) {
    campaign.products = await apiGetProductsForCampaign(campaign.id);
  }
  activeCampaigns.value = all.filter(c => c.is_active);
}

onMounted(async () => {
  products.value = await apiFetchProducts();
  salespeople.value = await apiFetchSalespeople();
  await loadActiveCampaigns();
});
</script>
