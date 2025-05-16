// src/database/products.rs

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub cost_price: f64,
    pub unit_price: f64,
    pub description: Option<String>,
    pub carton_size: i32, 
}

#[derive(Debug, Deserialize)]
pub struct NewProduct {
    pub sku: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub cost_price: f64,
    pub unit_price: f64,
    pub description: Option<String>,
    pub carton_size:i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub cost_price: f64,
    pub unit_price: f64,
    pub description: Option<String>,
    pub carton_size: i32,
}

#[tauri::command]
pub fn add_product(
    conn: State<'_, Mutex<Connection>>,
    product: NewProduct,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO products (sku, name, brand, category, cost_price, unit_price, description,carton_size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7,?8)",
        params![
            product.sku,
            product.name,
            product.brand,
            product.category,
            product.cost_price,
            product.unit_price,
            product.description,
            product.carton_size,
            
        ],
    ).map_err(|e| {
        println!("❌ DB add_product Error: {:?}", e);
        e.to_string()
    })?;

    Ok(())
}

#[tauri::command]
pub fn get_products(conn: State<'_, Mutex<Connection>>) -> Result<Vec<Product>, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, sku, name, brand, category, cost_price, unit_price, description,carton_size FROM products"
    ).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                sku: row.get(1)?,
                name: row.get(2)?,
                brand: row.get(3)?,
                category: row.get(4)?,
                cost_price: row.get(5)?,
                unit_price: row.get(6)?,
                description: row.get(7).ok(),
                carton_size: row.get(8)?,
            })
        })
        .map_err(|e| {
            println!("❌ DB get_products Error: {:?}", e);
            e.to_string()
        })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[tauri::command]
pub fn update_product(
    conn: State<'_, Mutex<Connection>>,
    product: UpdateProduct,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE products SET
            sku = ?1,
            name = ?2,
            brand = ?3,
            category = ?4,
            cost_price = ?5,
            unit_price = ?6,
            description = ?7,
            carton_size = ?8
         WHERE id = ?9",
        params![
            product.sku,
            product.name,
            product.brand,
            product.category,
            product.cost_price,
            product.unit_price,
            product.description,
            product.carton_size,
            product.id,
        ],
    )
    .map_err(|e| {
        println!("❌ DB update_product Error: {:?}", e);
        e.to_string()
    })?;

    Ok(())
}

#[tauri::command]
pub fn delete_product(
    conn: State<'_, Mutex<Connection>>,
    id: i32,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    // Check if product is used in salesentry
    let used_in_sales: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM salesentry WHERE product_id = ?1",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check salesentry usage: {}", e))?;

    // Check if product is used in target_campaign_products
    let used_in_campaigns: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM target_campaign_products WHERE product_id = ?1",
            params![id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check campaign usage: {}", e))?;

    // If product is in use, prevent deletion
    if used_in_sales > 0 || used_in_campaigns > 0 {
        return Err(format!(
            "❌ Cannot delete: Product is used in {} sale(s) and {} campaign(s).",
            used_in_sales, used_in_campaigns
        ));
    }

    // Safe to delete
    conn.execute(
        "DELETE FROM products WHERE id = ?1",
        params![id],
    )
    .map_err(|e| {
        println!("❌ DB delete_product Error: {:?}", e);
        e.to_string()
    })?;

    Ok(())
}

// this is not used, cz delete checks for usage in the above function
#[tauri::command]
pub fn check_product_usage(conn: State<'_, Mutex<Connection>>, product_id: i32) -> Result<bool, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    let mut stmt_campaigns = conn.prepare(
        "SELECT COUNT(*) FROM target_campaign_products WHERE product_id = ?1"
    ).map_err(|e| e.to_string())?;

    let campaign_count: i32 = stmt_campaigns.query_row(params![product_id], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let mut stmt_sales = conn.prepare(
        "SELECT COUNT(*) FROM salesentry WHERE product_id = ?1"
    ).map_err(|e| e.to_string())?;

    let sales_count: i32 = stmt_sales.query_row(params![product_id], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(campaign_count > 0 || sales_count > 0)
}
