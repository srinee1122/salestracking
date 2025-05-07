use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Salesperson {
    pub id: i32,
    pub name: String,
    pub employee_id: String,
    pub email: Option<String>,
    pub is_active: bool, // Now included to match frontend
}

#[tauri::command]
pub fn add_salesperson(
    conn: State<'_, Mutex<Connection>>,
    name: String,
    employee_id: String,
    email: Option<String>,
) -> Result<(), String> {
    let conn = conn.lock().map_err(|_| "Failed to lock DB")?;

    conn.execute(
        "INSERT INTO salespeople (name, employee_id, email, is_active) VALUES (?1, ?2, ?3, 1)",
        params![name, employee_id, email],
    )
    .map_err(|e| {
        println!("❌ DB insert error: {:?}", e);
        e.to_string()
    })?;

    Ok(())
}

#[tauri::command]
pub fn get_salespeople(
    conn: State<'_, Mutex<Connection>>,
) -> Result<Vec<Salesperson>, String> {
    let conn = conn.lock().map_err(|_| "Failed to lock DB")?;

    let mut stmt = conn
        .prepare("SELECT id, name, employee_id, email, is_active FROM salespeople")
        .map_err(|e| e.to_string())?;
  
    let rows = stmt
        .query_map([], |row| {
            Ok(Salesperson {
                id: row.get(0)?,
                name: row.get(1)?,
                employee_id: row.get(2)?,
                email: row.get(3).ok(),
                is_active: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for person in rows {
        result.push(person.map_err(|e| e.to_string())?);
    }

    Ok(result)
}
