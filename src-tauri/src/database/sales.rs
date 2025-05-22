use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleEntry {
    pub id: i32,
    pub salesperson_id: i32,
    pub date: String,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_type: String,
    pub brand: Option<String>,
    pub customer: Option<String>,
    pub sold_price: Option<f64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewSaleEntry {
    pub salesperson_id: i32,
    pub product_id: i32,
    pub date: String,
    pub quantity: i32,
    pub unit_type: String,
    pub brand: Option<String>,
    pub sold_price: Option<f64>,
    pub customer: Option<String>,
}

#[tauri::command]
pub fn add_sale_entry(
    conn: tauri::State<'_, std::sync::Mutex<rusqlite::Connection>>,
    entry: NewSaleEntry,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO salesentry (
            salesperson_id, product_id, date, quantity, unit_type, brand, sold_price, customer
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            entry.salesperson_id,
            entry.product_id,
            entry.date,
            entry.quantity,
            entry.unit_type,
            entry.brand,
            entry.sold_price,
            entry.customer,
        ],
    )
    .map_err(|e| {
        println!("❌ DB insert error: {:?}", e);
        e.to_string()
    })?;

    Ok(())
}

#[tauri::command]
pub fn get_sales_entries(conn: State<'_, Mutex<Connection>>) -> Result<Vec<SaleEntry>, String> {
    let conn = conn.lock().map_err(|_| "Failed to lock DB".to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, salesperson_id, date, product_id, quantity, unit_type,brand,customer,sold_price FROM salesentry ",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(SaleEntry {
                id: row.get(0)?,
                salesperson_id: row.get(1)?,
                date: row.get(2)?,
                product_id: row.get(3)?,
                quantity: row.get(4)?,
                unit_type: row.get(5)?,
                brand: row.get(6)?,
                customer: row.get(7)?,
                sold_price: row.get(8)?,

            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

#[tauri::command]
pub fn delete_sale_entry(conn: State<'_, Mutex<Connection>>, id: i32) -> Result<(), String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM salesentry WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
