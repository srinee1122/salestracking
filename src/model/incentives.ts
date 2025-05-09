// src/model/incentives.ts

import { invoke } from "@tauri-apps/api/core";

// --- Interfaces ---

//Name of the campaign 
export interface TargetCampaign {
  id: number;
  name: string;
  brand: string;
  start_date: string;
  end_date: string;
  is_active: boolean;
}

//products involved for the selected brand
export interface TargetCampaignProduct {
  id: number;
  campaign_id: number;
  product_id: number;
}

// setting minimum target
export interface TargetAllocation {
  id: number;
  campaign_id: number;
  salesperson_id: number;
  target_quantity: number;
  base_reward: number;
}

//include tier if needed
export interface TargetTier {
  id: number;
  campaign_id: number;
  min_quantity: number;
  multiplier: number;
  reward_per_unit: number;
  notes?: string;
}

// --- API Functions ---

export async function apiCreateCampaign(payload: Omit<TargetCampaign, 'id'>): Promise<void> {
  await invoke("add_target_campaign", { campaign: payload });
}

export async function apiGetCampaigns(): Promise<TargetCampaign[]> {
  return await invoke("get_target_campaigns");
}

export async function apiSetCampaignProducts(campaign_id: number, product_ids: number[]): Promise<void> {
  await invoke("set_campaign_products", { payload: { campaign_id, product_ids } });
}

export async function apiAddTargetAllocation(payload: Omit<TargetAllocation, 'id'>): Promise<void> {
  await invoke("add_target_allocation", { payload });
}

export async function apiAddTargetTier(payload: Omit<TargetTier, 'id'>): Promise<void> {
  await invoke("add_target_tier", { tier: payload });
}

export async function apiGetTargetAllocations(campaign_id: number): Promise<TargetAllocation[]> {
  console.log("enters get target allocation", campaign_id);
  return await invoke("get_target_allocations", {campaignId: campaign_id });
}

export async function apiGetTargetTiers(campaign_id: number): Promise<TargetTier[]> {
  return await invoke("get_target_tiers", { campaignId: campaign_id });
}
