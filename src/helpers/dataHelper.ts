// src/helpers/dataHelper.ts

import {
  apiGetCampaigns,
  apiCreateCampaign,
  apiSetCampaignProducts,
  apiAddTargetAllocation,
  apiAddTargetTier,
  apiGetTargetAllocations,
  apiGetTargetTiers,
  TargetCampaign,
  TargetAllocation,
  TargetTier,
} from '@/model/incentives';

import { apiFetchSalespeople } from '@/model/api';
import { apiFetchProducts } from '@/model/products';
import { apiFetchSaleEntries } from '@/model/sales';

export async function getCampaigns(): Promise<TargetCampaign[]> {
  try {
    return await apiGetCampaigns();
  } catch (err) {
    console.error('Error fetching campaigns:', err);
    return [];
  }
}

export async function getSalespeople(): Promise<any[]> {
  try {
    return await apiFetchSalespeople();
  } catch (err) {
    console.error('Error fetching salespeople:', err);
    return [];
  }
}

export async function getProducts(): Promise<any[]> {
  try {
    return await apiFetchProducts();
  } catch (err) {
    console.error('Error fetching products:', err);
    return [];
  }
}

export async function getSalesEntries(): Promise<any[]> {
  try {
    return await apiFetchSaleEntries();
  } catch (err) {
    console.error('Error fetching sale entries:', err);
    return [];
  }
}

export async function getTargetAllocations(campaign_id: number): Promise<TargetAllocation[]> {
  try {
    return await apiGetTargetAllocations(campaign_id);
  } catch (err) {
    console.error('Error fetching target allocations:', err);
    return [];
  }
}

export async function getTargetTiers(campaign_id: number): Promise<TargetTier[]> {
  try {
    return await apiGetTargetTiers(campaign_id);
  } catch (err) {
    console.error('Error fetching target tiers:', err);
    return [];
  }
}