// src/model/incentives.ts

import { invoke } from "@tauri-apps/api/core";

// --- Interfaces ---

export interface TargetCampaign {
  id: number;
  name: string;
  brand: string;
  start_date: string;
  end_date: string;
  is_active: boolean;
}

export interface TargetCampaignProduct {
  id: number;
  campaign_id: number;
  product_id: number;
}

export interface TargetAllocation {
  id: number;
  campaign_id: number;
  salesperson_id: number;
  target_quantity: number;
}

export interface TargetTier {
  id: number;
  campaign_id: number;
  min_quantity: number;
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

export async function apiAddTargetAllocation(payload: Omit<TargetAllocation, 'id'>): Promise<void> {
  await invoke("add_target_allocation", { payload }); // ✅ Matches Rust command
}

export async function apiAddTargetTier(payload: Omit<TargetTier, 'id'>): Promise<void> {
  await invoke("add_target_tier", { tier: payload });
}

export async function apiSetCampaignProducts(campaignId: number, productIds: number[]): Promise<void> {
  await invoke("set_campaign_products", {
    payload: { campaign_id: campaignId, product_ids: productIds }
  });
}
