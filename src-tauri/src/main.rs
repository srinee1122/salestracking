#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use database::db::establish_connection;
use database::product::{add_product,get_products};
use database::salesperson::{add_salesperson, get_salespeople};
use database::sales::{add_sale_entry, get_sales_entries};
use database::incentives::{add_target_campaign, get_target_campaigns, add_target_allocation, add_target_tier, set_campaign_products};

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
             get_products,
             add_sale_entry,
             get_sales_entries,
             add_product,
             get_products,
             add_salesperson,
             get_salespeople,
             add_sale_entry,
             get_sales_entries,
             add_target_campaign,
             get_target_campaigns,
             add_target_allocation,
             add_target_tier,
             set_campaign_products
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
