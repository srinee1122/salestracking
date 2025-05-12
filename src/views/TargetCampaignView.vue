<template>
  <div class="p-6 sm:p-8">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">🎯 Target Campaign Management</h1>

      <!-- ✅ Button to toggle form -->
  

    <!-- ✅ ACTIVE CAMPAIGNS prominently shown -->
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
  <div class="text-right mb-4">
      <button
        class="bg-green-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg text-lg shadow-md"
        @click="showForm = !showForm"
      >
         Create New Campaign
      </button>
    </div>
        <!-- ✅ Campaign Creation Form (conditionally shown) -->
    <section v-if="showForm" class="bg-white p-6 rounded-lg shadow mb-8 border border-blue-300">
      <h2 class="text-xl font-semibold text-gray-700 mb-4">🆕 Create New Campaign</h2>
    <!-- Step 1: Campaign Info -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">1. Campaign Info</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <input v-model="campaignForm.name"  class="input border border-gray-300 text-center rounded px-3 py-2 w-full ":class="{
    'border-red-500 border-2': showErrors && !campaignForm.name,
    'border-gray-300': !(showErrors && !campaignForm.name) }" placeholder="Campaign Name" required/>

        <select v-model="campaignForm.brand" class="input border border-gray-300 text-center rounded px-3 py-2 w-full" required>
          <option disabled value="">Select Brand</option>
          <option v-for="brand in uniqueBrands" :key="brand" :value="brand">{{ brand }}</option>
        </select>
        <input type="date" v-model="campaignForm.start_date" class="input  border border-gray-300 text-center rounded px-3 py-2 w-full" :class="{
    'border-red-500 border-2': showErrors && !campaignForm.start_date,
    'border-gray-300': !(showErrors && !campaignForm.name) }" required />
        <input type="date" v-model="campaignForm.end_date" class="input  border border-gray-300 text-center rounded px-3 py-2 w-full" :class="{
    'border-red-500 border-2': showErrors && !campaignForm.end_date,
    'border-gray-300': !(showErrors && !campaignForm.name) }"required />
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
        <label class="font-medium border border-gray-300 text-center rounded px-3 py-2 w-full">{{ person.name }}</label>
        <input type="number" v-model.number="salesTargets[person.id]" placeholder="Min Qty" class="input border border-gray-300 text-center rounded px-3 py-2 w-full" :class="{
    'border-red-500 border-2': showErrors && !salesTargets[person.id],
    'border-gray-300': !(showErrors && !salesTargets[person.id]) }" />
        <input type="number" v-model.number="baseRewards[person.id]" placeholder="Base Reward/Unit" class="input border border-gray-300 text-center rounded px-3 py-2 w-full" :class="{
    'border-red-500 border-2': showErrors && !baseRewards[person.id],
    'border-gray-300': !(showErrors && !baseRewards[person.id]) }"/>

         <select v-model.text = "targetunits[person.id]"  class="input border border-gray-300 text-center rounded px-3 py-2 w-full" :class="{
    'border-red-500 border-2': showErrors && !targetunits[person.id],
    'border-gray-300': !(showErrors && !targetunits[person.id]) }">
          <option disabled value="">Select unit type</option>
  <option value="pieces">Pieces</option>
  <option value="cartons">Cartons</option>
</select> 

      </div>
    </section>


 

    <!-- Step 4: Define Tier Multipliers -->
    <section class="bg-white p-6 rounded-lg shadow mb-8">
      <h2 class="text-lg font-semibold text-gray-700 mb-4">4. Incentive Tiers (Multipliers)</h2>
      <form @submit.prevent="addTier" class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-4">
        <section>
           <label class=" border-gray-300 text-center rounded px-3 py-2 w-full">Multiplier</label>
          <input type="number" step="0.1" v-model.number="tierForm.multiplier" placeholder="Multiplier" class="input border border-gray-300 text-center rounded px-3 py-2 w-full"  required />
      </section>
        <section>
          <label class=" border-gray-300 text-center rounded px-3 py-2 w-full">Reward amount</label>
<input type="number" v-model.number="tierForm.reward_per_unit" placeholder="Reward/Unit" class="input border border-gray-300 text-center rounded px-3 py-2 w-full" required />
        </section>
        
        <section>
          <label class=" border-gray-300 text-center rounded px-3 py-2 w-full">Name Of The Tier</label>
        <input type="text" v-model="tierForm.notes" placeholder="Tier Label (optional)" class="input border border-gray-300 text-center rounded px-3 py-2 w-full" />
        </section>
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
        </section>

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
const showForm = ref(false);


const campaignForm = ref({ name: '', brand: '', start_date: '', end_date: '' });
const products = ref<any[]>([]);
const salespeople = ref<any[]>([]);
const selectedProducts = ref<number[]>([]);
const salesTargets = ref<Record<number, number>>({});
const baseRewards = ref<Record<number, number>>({});
const targetunits = ref<Record<string, string>>({});
const tierForm = ref({ multiplier: 1, reward_per_unit: 1, notes: '' });
const tierList = ref<any[]>([]);
const activeCampaigns = ref<TargetCampaign[]>([]);
const showErrors = ref(false);

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

   if (tierList.value.length >= 3) {
    alert('⚠️ You can only add up to 3 tiers.');
    return;
  }
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
  showErrors.value = true;
  if (!campaignForm.value.name || !campaignForm.value.brand || !campaignForm.value.start_date || !campaignForm.value.end_date) {
  alert('❌ Please fill all campaign info fields.');
  return;
}

if (new Date(campaignForm.value.end_date) <= new Date(campaignForm.value.start_date)) {
  alert('❌ End date must be after start date.');
  return;
}

if (selectedProducts.value.length === 0) {
  alert('❌ Please select at least one product.');
  return;
}

for (const person of salespeople.value) {
  const qty = salesTargets.value[person.id] || 0;
  const reward = baseRewards.value[person.id] || 0;
  const unit = targetunits.value[person.id];
  if (qty <= 0 || reward <= 0 || (unit !== 'pieces' && unit !== 'cartons')) {
    alert(`❌ Check target/reward/unit for ${person.name}`);
    return;
  }
}

if (tierList.value.length > 0) {
  for (const tier of tierList.value) {
    if (!tier.multiplier || tier.multiplier <= 0 || !tier.reward_per_unit || tier.reward_per_unit <= 0) {
      alert(`❌ Please ensure all tier fields are filled correctly.`);
      return;
    }
  }
}
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
      const tgtunit = targetunits.value[person.id] || "pieces";
      if (qty > 0 && reward > 0) {
        await apiAddTargetAllocation({
          campaign_id: newCampaign.id,
          salesperson_id: person.id,
          target_quantity: qty,
          base_reward: reward,
          target_unit : tgtunit,
        });
      }
    }

    // Save incentive tiers
for (const tier of tierList.value) {
  await apiAddTargetTier({
    campaign_id: newCampaign.id,
    multiplier: tier.multiplier , // <- enforce it
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
