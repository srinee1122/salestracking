// src/database/db.rs

use tauri::AppHandle; // Use tauri's AppHandle to access path resolver
use std::path::PathBuf;
use rusqlite::{Connection, Error as RusqliteError, ffi}; // Import rusqlite types
use tauri::Manager;
use std::fs;

use super::schema::initialize_database; 
// --- Helper function to get the database path ---
// This function now takes AppHandle and returns Result<PathBuf, tauri::Error>
fn get_database_path(handle: &AppHandle) -> Result<PathBuf, tauri::Error> {
    // 1. Get the path to the app's data directory
    let app_data_dir = handle.path().app_data_dir()?;
    println!("App data directory reported by Tauri: {:?}", app_data_dir); // Log the base dir

    // 2. --- Ensure the directory exists ---
    fs::create_dir_all(&app_data_dir).map_err(|io_err| {
        // If creating the directory fails (e.g., permissions), log the error
        eprintln!("Failed to create app data directory at {:?}: {}", &app_data_dir, io_err);
        // Convert the std::io::Error into a tauri::Error::Io variant
        tauri::Error::Io(io_err)
    })?; // Propagate error if directory creation fails
    println!("App data directory exists or was created: {:?}", app_data_dir); // Confirm directory creation/existence

    // 3. Define the database filename (make sure this is the name you want)
    let db_file_name = "sales_tracker.db";
    let db_path = app_data_dir.join(db_file_name);

    println!("Full database path: {:?}", db_path); // Log the final path
    Ok(db_path)
}

// --- Public function to establish the database connection ---
// This function handles the potential error from get_database_path
pub fn establish_connection(app_handle: &AppHandle) -> Result<Connection, RusqliteError> {
    let database_path = get_database_path(app_handle).map_err(|tauri_err| {
        eprintln!("Tauri error getting database path: {}", tauri_err);
        RusqliteError::SqliteFailure(
            ffi::Error::new(ffi::SQLITE_CANTOPEN),
            Some(format!("Setup error: Failed to determine database path - {}", tauri_err))
        )
    })?;

    let mut conn = Connection::open(&database_path).map_err(|sqlite_err| {
        eprintln!("SQLite error opening database at {:?}: {}", database_path, sqlite_err);
        sqlite_err
    })?;

    // ✅ Initialize schema after connection
    initialize_database(&mut conn).map_err(|e| {
        eprintln!("Failed to initialize database schema: {}", e);
        RusqliteError::ExecuteReturnedResults
    })?;

    Ok(conn)
}