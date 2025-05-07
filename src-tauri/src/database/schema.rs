// src/database/schema.rs

use rusqlite::{Connection, Result as RusqliteResult};

/// Initializes only the tables currently in use: `salespeople` and `products`.
pub fn initialize_database(conn: &mut Connection) -> RusqliteResult<()> {
    println!("Initializing database schema (salespeople + products)...");

    let tx = conn.transaction()?;

    // Create salespeople table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS salespeople (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            employee_id TEXT,         -- Removed UNIQUE constraint
            email TEXT,
            is_active INTEGER DEFAULT 1 NOT NULL
        )",
        [],
    )?;
    println!("✅ Table 'salespeople' checked/created.");

    // Create products table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sku TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            brand TEXT NOT NULL,
            category TEXT,
            cost_price REAL NOT NULL,
            unit_price REAL NOT NULL,
            description TEXT
        )",
        [],
    )?;
    println!("✅ Table 'products' checked/created.");

    tx.commit()?;
    println!("✅ Database schema initialization complete.");

    Ok(())
}
