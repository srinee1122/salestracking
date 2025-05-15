#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use database::db::establish_connection;
use database::product::{add_product,get_products,update_product,delete_product};
use database::salesperson::{add_salesperson, get_salespeople};
use database::sales::{add_sale_entry, get_sales_entries};
use database::incentives::{add_target_campaign, get_target_campaigns, add_target_allocation, add_target_tier, set_campaign_products, get_target_allocations, get_target_tiers, get_products_for_campaign};

use std::sync::Mutex;
use tauri::{AppHandle, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db_connection = establish_connection(&app.handle())
                .expect("Failed to connect to DB");
            app.manage(Mutex::new(db_connection));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
             add_product,
             add_salesperson,
             get_salespeople,
             add_sale_entry,
             get_sales_entries,
             get_products,
             get_sales_entries,
             add_target_campaign,
             get_target_campaigns,
             get_target_allocations,
             add_target_allocation,
             add_target_tier,
             set_campaign_products,
             get_target_tiers,
             get_products_for_campaign,
             update_product,
             delete_product
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
