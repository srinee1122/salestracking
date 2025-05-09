// src/database/incentives.rs

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetCampaign {
    pub id: i32,
    pub name: String,
    pub brand: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTargetCampaign {
    pub name: String,
    pub brand: String,
    pub start_date: String,
    pub end_date: String,
}

#[tauri::command]
pub fn add_target_campaign(
    conn: State<'_, Mutex<Connection>>,
    campaign: NewTargetCampaign,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO target_campaigns (name, brand, start_date, end_date, is_active)
         VALUES (?1, ?2, ?3, ?4, 1)",
        params![
            campaign.name,
            campaign.brand,
            campaign.start_date,
            campaign.end_date
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_target_campaigns(
    conn: State<'_, Mutex<Connection>>,
) -> Result<Vec<TargetCampaign>, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, brand, start_date, end_date, is_active FROM target_campaigns")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(TargetCampaign {
                id: row.get(0)?,
                name: row.get(1)?,
                brand: row.get(2)?,
                start_date: row.get(3)?,
                end_date: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[derive(Debug, Deserialize)]
pub struct TargetAllocationPayload {
    pub campaign_id: i32,
    pub salesperson_id: i32,
    pub target_quantity: i32,
    pub base_reward: f64,
}

#[tauri::command]
pub fn add_target_allocation(
    conn: State<'_, Mutex<Connection>>,
    payload: TargetAllocationPayload,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO target_allocations (campaign_id, salesperson_id, target_quantity, base_reward)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            payload.campaign_id,
            payload.salesperson_id,
            payload.target_quantity,
            payload.base_reward
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct TargetTierPayload {
    pub campaign_id: i32,
    pub min_quantity: i32,
    pub reward_per_unit: f64,
    pub notes: Option<String>,
}

#[tauri::command]
pub fn add_target_tier(
    conn: State<'_, Mutex<Connection>>,
    tier: TargetTierPayload,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO target_tiers (campaign_id, min_quantity, reward_per_unit, notes)
         VALUES (?1, ?2, ?3, ?4)",
        params![tier.campaign_id, tier.min_quantity, tier.reward_per_unit, tier.notes],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct CampaignProductAssignment {
    pub campaign_id: i32,
    pub product_ids: Vec<i32>,
}

#[tauri::command]
pub fn set_campaign_products(
    conn: State<'_, Mutex<Connection>>,
    payload: CampaignProductAssignment,
) -> Result<(), String> {
    let mut conn = conn.lock().map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "DELETE FROM target_campaign_products WHERE campaign_id = ?1",
        params![payload.campaign_id],
    )
    .map_err(|e| e.to_string())?;

    for product_id in payload.product_ids {
        tx.execute(
            "INSERT INTO target_campaign_products (campaign_id, product_id)
             VALUES (?1, ?2)",
            params![payload.campaign_id, product_id],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct TargetAllocation {
    pub id: i32,
    pub campaign_id: i32,
    pub salesperson_id: i32,
    pub target_quantity: i32,
    pub base_reward: f64,
}

#[tauri::command]
pub fn get_target_allocations(
    conn: State<'_, Mutex<Connection>>,
    campaign_id: i32,
) -> Result<Vec<TargetAllocation>, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    println!("🟢 Rust: get_target_allocations called with ID: {}", campaign_id);
    println!("Rust received campaign_id = {}", campaign_id);
    let mut stmt = conn
        .prepare(
            "SELECT id, campaign_id, salesperson_id, target_quantity, base_reward
             FROM target_allocations
             WHERE campaign_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([campaign_id], |row| {
            Ok(TargetAllocation {
                id: row.get(0)?,
                campaign_id: row.get(1)?,
                salesperson_id: row.get(2)?,
                target_quantity: row.get(3)?,
                base_reward: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[derive(Debug, Serialize)]
pub struct TargetTier {
    pub id: i32,
    pub campaign_id: i32,
    pub min_quantity: i32,
    pub multiplier: f64,
    pub reward_per_unit: f64,
    pub notes: Option<String>,
}

#[tauri::command]
pub fn get_target_tiers(
    conn: State<'_, Mutex<Connection>>,
    campaign_id: i32,
) -> Result<Vec<TargetTier>, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, campaign_id, min_quantity, multiplier, reward_per_unit, notes
         FROM target_tiers
         WHERE campaign_id = ?1",
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([campaign_id], |row| {
        Ok(TargetTier {
            id: row.get(0)?,
            campaign_id: row.get(1)?,
            min_quantity: row.get(2)?,
            multiplier: row.get(3)?,
            reward_per_unit: row.get(4)?,
            notes: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut tiers = Vec::new();
    for row in rows {
        tiers.push(row.map_err(|e| e.to_string())?);
    }

    Ok(tiers)
}

