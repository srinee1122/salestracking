// src/commands.rs

// --- Imports ---
use tauri::State; // To access managed state
use std::sync::Mutex; // To access the Mutex guarded Connection
// Removed unused `Result as RusqliteResult`
use rusqlite::{params, Connection, Error as RusqliteError};

// Define the type alias for the state managed in main.rs
// Assuming main.rs has `pub type DbState = Mutex<Connection>;`
// If not public in main.rs, define it here:
type DbState = Mutex<Connection>;

// --- Error Handling Helper ---
fn map_db_error(err: RusqliteError) -> String {
    eprintln!("Database error: {}", err);
    err.to_string()
}

// --- Tauri Commands ---

// Command to add a new salesperson
#[tauri::command]
// Make the function public
pub async fn add_salesperson(
    name: String,
    employee_id: Option<String>,
    email: Option<String>,
    db_state: State<'_, DbState>,
) -> Result<(), String> {
    println!(
        "Received add_salesperson command: Name='{}', EmployeeID='{:?}', Email='{:?}'",
        name, employee_id, email
    );

    let conn_guard = db_state.lock().map_err(|_| "Database lock poisoned".to_string())?;

    conn_guard.execute(
        "INSERT INTO salespeople (name, employee_id, email, is_active) VALUES (?1, ?2, ?3, 1)",
        params![name, employee_id, email],
    )
    .map(|rows_affected| {
        println!("Salesperson added successfully. Rows affected: {}", rows_affected);
        ()
    })
    .map_err(map_db_error)?;

    Ok(())
}

// --- Example Command to Fetch Salespeople ---
#[derive(serde::Serialize)]
pub struct Salesperson {
    id: i32,
    name: String,
    employee_id: Option<String>,
    email: Option<String>,
    is_active: bool,
}

#[tauri::command]
// Make the function public
pub async fn get_salespeople(db_state: State<'_, DbState>) -> Result<Vec<Salesperson>, String> {
    println!("Received get_salespeople command");

    let conn_guard = db_state.lock().map_err(|_| "Database lock poisoned".to_string())?;

    let mut stmt = conn_guard.prepare("SELECT id, name, employee_id, email, is_active FROM salespeople")
        .map_err(map_db_error)?;

    let people_iter = stmt.query_map([], |row| {
        Ok(Salesperson {
            id: row.get(0)?,
            name: row.get(1)?,
            employee_id: row.get(2)?,
            email: row.get(3)?,
            is_active: row.get::<_, i32>(4)? == 1,
        })
    }).map_err(map_db_error)?;

    let mut people = Vec::new();
    for person_result in people_iter {
        people.push(person_result.map_err(map_db_error)?);
    }

    println!("Returning {} salespeople.", people.len());
    Ok(people)
}
