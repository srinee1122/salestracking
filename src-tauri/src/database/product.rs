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
//hekkww