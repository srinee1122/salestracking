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
}

#[derive(Debug, Deserialize)]
pub struct NewSaleEntry {
    pub salesperson_id: i32,
    pub date: String,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_type: String,
}

#[tauri::command]
pub fn add_sale_entry(
    conn: State<'_, Mutex<Connection>>,
    entry: NewSaleEntry,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|_| "Failed to lock DB".to_string())?;
    println!("📥 Adding sale entry: {:?}", entry);
    conn.execute(
        "INSERT INTO sales_entries (salesperson_id, date, product_id, quantity, unit_type)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            entry.salesperson_id,
            entry.date,
            entry.product_id,
            entry.quantity,
            entry.unit_type
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_sales_entries(conn: State<'_, Mutex<Connection>>) -> Result<Vec<SaleEntry>, String> {
    let conn = conn.lock().map_err(|_| "Failed to lock DB".to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, salesperson_id, date, product_id, quantity, unit_type FROM sales_entries",
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
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}
